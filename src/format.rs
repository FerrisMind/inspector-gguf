use candle::quantized::gguf_file;
use std::fs::File;
use std::io::Read;

pub fn load_gguf_metadata_sync(
    path: &std::path::Path,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    puffin::profile_scope!("load_gguf_metadata_sync");

    let mut f = {
        puffin::profile_scope!("file_open");
        File::open(path)?
    };

    let mut buf = Vec::new();
    {
        puffin::profile_scope!("file_reading");
        f.read_to_end(&mut buf)?;
    }

    let content = {
        puffin::profile_scope!("gguf_parsing");
        let mut cursor = std::io::Cursor::new(&buf);
        candle::quantized::gguf_file::Content::read(&mut cursor)?
    };

    // Read header fields from the buffer (candle may have moved the cursor)
    let header_fields = read_gguf_header_from_buffer(&buf).unwrap_or_else(|e| {
        eprintln!("ERROR reading header: {}", e);
        GGufHeader { version: 0, tensor_count: 0, kv_count: 0 }
    });

    let mut out = Vec::new();
    {
        puffin::profile_scope!("metadata_processing");

        // Add header fields first

        out.push(("version".to_string(), header_fields.version.to_string()));
        out.push(("tensor_count".to_string(), header_fields.tensor_count.to_string()));
        out.push(("kv_count".to_string(), header_fields.kv_count.to_string()));

        // Add metadata
        for (k, v) in content.metadata.iter() {
            let s = readable_value_for_key(k, v);
            out.push((k.clone(), s));
        }
    }

    Ok(out)
}

#[allow(clippy::type_complexity)]
pub fn load_gguf_metadata_with_full_content_sync(
    path: &std::path::Path,
) -> Result<Vec<(String, String, Option<String>)>, Box<dyn std::error::Error>> {
    puffin::profile_scope!("load_gguf_metadata_with_full_content_sync");

    let mut f = {
        puffin::profile_scope!("file_open");
        File::open(path)?
    };

    let mut buf = Vec::new();
    {
        puffin::profile_scope!("file_reading");
        f.read_to_end(&mut buf)?;
    }

    let content = {
        puffin::profile_scope!("gguf_parsing");
        let mut cursor = std::io::Cursor::new(&buf);
        candle::quantized::gguf_file::Content::read(&mut cursor)?
    };

    // Read header fields from the buffer (candle may have moved the cursor)
    let header_fields = read_gguf_header_from_buffer(&buf).unwrap_or_else(|e| {
        eprintln!("ERROR reading header: {}", e);
        GGufHeader { version: 0, tensor_count: 0, kv_count: 0 }
    });

    let mut out = Vec::new();
    {
        puffin::profile_scope!("metadata_processing");

        // Add header fields first

        out.push(("version".to_string(), header_fields.version.to_string(), None));
        out.push(("tensor_count".to_string(), header_fields.tensor_count.to_string(), None));
        out.push(("kv_count".to_string(), header_fields.kv_count.to_string(), None));

        // Add metadata
        for (k, v) in content.metadata.iter() {
            let s = readable_value_for_key(k, v);
            let full_content = get_full_tokenizer_content(k, v);
            out.push((k.clone(), s, full_content));
        }
    }

    Ok(out)
}

#[derive(Debug)]
struct GGufHeader {
    version: u32,
    tensor_count: u64,
    kv_count: u64,
}

fn read_gguf_header_from_buffer(buffer: &[u8]) -> Result<GGufHeader, Box<dyn std::error::Error>> {
    if buffer.len() < 20 {
        return Err("Buffer too small for GGUF header".into());
    }

    // Check magic bytes "GGUF" (first 4 bytes)
    if &buffer[0..4] != b"GGUF" {
        return Err("Invalid GGUF magic bytes".into());
    }

    // Read version (uint32, little endian) - bytes 4-7
    let version = u32::from_le_bytes(buffer[4..8].try_into()?);

    // Read tensor count (uint64, little endian) - bytes 8-15
    let tensor_count = u64::from_le_bytes(buffer[8..16].try_into()?);

    // Read kv count (uint64, little endian) - bytes 16-23
    let kv_count = u64::from_le_bytes(buffer[16..24].try_into()?);


    Ok(GGufHeader {
        version,
        tensor_count,
        kv_count,
    })
}

#[allow(clippy::collapsible_if)]
pub fn readable_value_for_key(key: &str, v: &gguf_file::Value) -> String {
    readable_value_for_key_full(key, v, false)
}

pub fn readable_value_for_key_full(key: &str, v: &gguf_file::Value, full_content: bool) -> String {
    // Special handling for tokenizer.chat_template - decode as UTF-8 string instead of base64
    if key == "tokenizer.chat_template"
        && let gguf_file::Value::Array(arr) = v
        && !arr.is_empty() && arr.iter().all(|el| matches!(el, gguf_file::Value::U8(_))) {
        let bytes: Vec<u8> = arr.iter()
            .filter_map(|el| {
                if let gguf_file::Value::U8(b) = el {
                    Some(*b)
                } else {
                    None
                }
            })
            .collect();
        if let Ok(s) = String::from_utf8(bytes) {
            return s;
        }
    }

    // Special handling for tokenizer.ggml.tokens and tokenizer.ggml.merges - decode arrays of strings
    if (key == "tokenizer.ggml.tokens" || key == "tokenizer.ggml.merges")
        && let gguf_file::Value::Array(arr) = v
        && !arr.is_empty() {
        // Try to decode as array of strings (each element should be a string value)
        let mut strings = Vec::new();
        for el in arr.iter() {
            match el {
                gguf_file::Value::String(s) => {
                    strings.push(s.clone());
                }
                gguf_file::Value::Array(inner_arr) => {
                    // Fallback: try to decode as array of bytes
                    if !inner_arr.is_empty() && inner_arr.iter().all(|iel| matches!(iel, gguf_file::Value::U8(_))) {
                        let bytes: Vec<u8> = inner_arr.iter()
                            .filter_map(|iel| {
                                if let gguf_file::Value::U8(b) = iel {
                                    Some(*b)
                                } else {
                                    None
                                }
                            })
                            .collect();
                        if let Ok(s) = String::from_utf8(bytes) {
                            strings.push(s);
                        }
                    }
                }
                _ => {
                    // Other types - just convert to string representation
                    strings.push(format!("{:?}", el));
                }
            }
        }
        if !strings.is_empty() {
            if strings.len() <= 5 || full_content {
                return strings.join(", ");
            } else {
                let first_few = strings.iter().take(3).cloned().collect::<Vec<_>>().join(", ");
                return format!("{}, …", first_few);
            }
        }
    }

    // Special handling for arrays - show examples like in real.md
    if let gguf_file::Value::Array(arr) = v {
        // If array of bytes (U8) - for tokenizer data, show as string preview
        if !arr.is_empty() && arr.iter().all(|el| matches!(el, gguf_file::Value::U8(_))) {
            let bytes_len = arr.len();
            if bytes_len <= 64 {
                // Try to decode as UTF-8 string
                let bytes: Vec<u8> = arr.iter()
                    .filter_map(|el| {
                        if let gguf_file::Value::U8(b) = el {
                            Some(*b)
                        } else {
                            None
                        }
                    })
                    .collect();
                if let Ok(s) = String::from_utf8(bytes.clone()) {
                    // Show first part of the string
                    if s.len() <= 50 {
                        return s;
                    } else {
                        return format!("{}…", &s[..50]);
                    }
                } else {
                    // If not valid UTF-8, show as hex
                    let hex: String = bytes.iter().take(20).map(|b| format!("{:02x}", b)).collect();
                    return format!("{}…", hex);
                }
            } else {
                return format!("Array(len={})", bytes_len);
            }
        }

        // For other arrays, show first few elements like in real.md
        if arr.len() <= 10 {
            let parts: Vec<String> = arr.iter().map(|el| format!("{:?}", el)).collect();
            return format!("{}, …", parts.join(", "));
        } else {
            // Show first 3 elements and indicate there are more
            let first_parts: Vec<String> = arr.iter().take(3).map(|el| format!("{:?}", el)).collect();
            return format!("{}, …", first_parts.join(", "));
        }
    }

    // For scalar values, try the library-provided string representation
    if let Ok(s) = v.to_string() {
        return s.to_string();
    }

    // Fallback to debug representation
    format!("{:?}", v)
}

pub fn get_full_tokenizer_content(key: &str, v: &gguf_file::Value) -> Option<String> {
    // For tokenizer keys, return the full readable value without truncation
    if key.starts_with("tokenizer.") {
        Some(readable_value_for_key_full(key, v, true))
    } else {
        None
    }
}

pub fn readable_value(v: &gguf_file::Value) -> String {
    readable_value_for_key("", v)
}
