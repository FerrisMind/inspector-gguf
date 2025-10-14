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

    let mut out = Vec::new();
    {
        puffin::profile_scope!("metadata_processing");
        for (k, v) in content.metadata.iter() {
            let s = readable_value(v);
            out.push((k.clone(), s));
        }
    }

    Ok(out)
}

#[allow(clippy::collapsible_if)]
pub fn readable_value_for_key(key: &str, v: &gguf_file::Value) -> String {
    // Special handling for tokenizer.chat_template - decode as UTF-8 string instead of hex
    if key == "tokenizer.chat_template" {
        if let gguf_file::Value::Array(arr) = v {
            if !arr.is_empty() && arr.iter().all(|el| matches!(el, gguf_file::Value::U8(_))) {
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
        }
    }

    // Prefer the library-provided string representation when available
    if let Ok(s) = v.to_string() {
        return s.to_string();
    }

    // If it's an array, try to produce a compact textual representation.
    // Special-case: arrays of U8 should be printed as hex (blob-like).
    match v {
        gguf_file::Value::Array(arr) => {
            // If array of bytes (U8) -> hex preview or length
            if !arr.is_empty() && arr.iter().all(|el| matches!(el, gguf_file::Value::U8(_))) {
                let bytes_len = arr.len();
                if bytes_len <= 64 {
                    let hex: String = arr
                        .iter()
                        .map(|el| {
                            if let gguf_file::Value::U8(b) = el {
                                format!("{:02x}", b)
                            } else {
                                String::new()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("");
                    return hex;
                } else {
                    return format!("Blob(len={})", bytes_len);
                }
            }

            // Map small arrays to joined string, large arrays -> debug
            if arr.len() <= 64 {
                let parts: Vec<String> = arr.iter().map(|el| format!("{:?}", el)).collect();
                format!("Array([{}])", parts.join(", "))
            } else {
                format!("Array(len={})", arr.len())
            }
        }
        _ => format!("{:?}", v),
    }
}

pub fn readable_value(v: &gguf_file::Value) -> String {
    readable_value_for_key("", v)
}
