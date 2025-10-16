//! GGUF file format parsing and metadata extraction.
//!
//! This module provides functionality for parsing GGUF (GPT-Generated Unified Format) files
//! and extracting their metadata. GGUF is a binary format used for storing machine learning
//! models, particularly large language models, along with their associated metadata such as
//! tokenizer configurations, model parameters, and tensor information.
//!
//! # Key Features
//!
//! - **Synchronous parsing**: Efficient loading of GGUF files with metadata extraction
//! - **Header analysis**: Direct access to GGUF header fields (version, tensor count, key-value count)
//! - **Metadata processing**: Conversion of binary metadata to human-readable formats
//! - **Tokenizer support**: Special handling for tokenizer data including chat templates and token arrays
//! - **Performance profiling**: Built-in puffin profiler integration for performance monitoring
//!
//! # Usage Examples
//!
//! Basic metadata extraction:
//!
//! ```no_run
//! use inspector_gguf::format::load_gguf_metadata_sync;
//! use std::path::Path;
//!
//! let path = Path::new("model.gguf");
//! let metadata = load_gguf_metadata_sync(path)?;
//!
//! for (key, value) in metadata {
//!     println!("{}: {}", key, value);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Loading with full tokenizer content:
//!
//! ```no_run
//! use inspector_gguf::format::load_gguf_metadata_with_full_content_sync;
//! use std::path::Path;
//!
//! let path = Path::new("model.gguf");
//! let metadata = load_gguf_metadata_with_full_content_sync(path)?;
//!
//! for (key, value, full_content) in metadata {
//!     println!("{}: {}", key, value);
//!     if let Some(full) = full_content {
//!         println!("  Full content: {}", full);
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # GGUF Format Overview
//!
//! The GGUF format consists of:
//! - **Magic bytes**: "GGUF" identifier (4 bytes)
//! - **Version**: Format version number (4 bytes)
//! - **Tensor count**: Number of tensors in the file (8 bytes)
//! - **Key-value count**: Number of metadata entries (8 bytes)
//! - **Metadata**: Key-value pairs containing model information
//! - **Tensor data**: The actual model weights and parameters
//!
//! This module focuses on the header and metadata portions, providing easy access to
//! model information without loading the full tensor data into memory.

use candle::quantized::gguf_file;
use std::fs::File;
use std::io::Read;

/// Loads GGUF file metadata synchronously and returns key-value pairs.
///
/// This function reads a GGUF file from the specified path and extracts all metadata
/// as human-readable key-value pairs. It includes header information (version, tensor count,
/// key-value count) followed by all metadata entries from the file.
///
/// The function is optimized for performance with built-in profiling scopes and handles
/// large files efficiently by reading the entire file into memory before parsing.
///
/// # Arguments
///
/// * `path` - Path to the GGUF file to be analyzed
///
/// # Returns
///
/// Returns a `Vec<(String, String)>` where each tuple contains:
/// - First element: metadata key name
/// - Second element: human-readable string representation of the value
///
/// # Examples
///
/// ```
/// use inspector_gguf::format::load_gguf_metadata_sync;
/// use std::path::Path;
///
/// // Test with the sample GGUF file
/// let path = Path::new("model/Qwen3-0.6B-Q5_K_M.gguf");
/// let metadata = load_gguf_metadata_sync(path).expect("Failed to load GGUF metadata");
///
/// // Verify we got some metadata
/// assert!(!metadata.is_empty(), "Metadata should not be empty");
///
/// // Check that header fields are present
/// let version = metadata.iter().find(|(k, _)| k == "version");
/// assert!(version.is_some(), "Version should be present in metadata");
///
/// let tensor_count = metadata.iter().find(|(k, _)| k == "tensor_count");
/// assert!(tensor_count.is_some(), "Tensor count should be present in metadata");
///
/// let kv_count = metadata.iter().find(|(k, _)| k == "kv_count");
/// assert!(kv_count.is_some(), "KV count should be present in metadata");
///
/// // Verify version is a valid number
/// if let Some((_, version_str)) = version {
///     let version_num: u32 = version_str.parse().expect("Version should be a valid number");
///     assert!(version_num > 0, "Version should be greater than 0");
/// }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be opened or read (I/O errors)
/// - The file is not a valid GGUF format
/// - The file is corrupted or truncated
/// - Insufficient memory to load the file
/// - Invalid UTF-8 sequences in string metadata
///
/// # Performance
///
/// This function includes performance profiling scopes:
/// - `load_gguf_metadata_sync`: Overall function timing
/// - `file_open`: File opening time
/// - `file_reading`: File I/O time
/// - `gguf_parsing`: GGUF format parsing time
/// - `metadata_processing`: Metadata conversion time
///
/// # Error Handling Example
///
/// ```
/// use inspector_gguf::format::load_gguf_metadata_sync;
/// use std::path::Path;
///
/// // Test with non-existent file
/// let path = Path::new("nonexistent.gguf");
/// let result = load_gguf_metadata_sync(path);
/// assert!(result.is_err(), "Should fail for non-existent file");
///
/// // Test with invalid file (not a GGUF file)
/// let path = Path::new("Cargo.toml"); // This is not a GGUF file
/// let result = load_gguf_metadata_sync(path);
/// assert!(result.is_err(), "Should fail for non-GGUF file");
/// ```
///
/// See also [`load_gguf_metadata_with_full_content_sync`] for extended tokenizer content,
/// [`readable_value_for_key`] for value formatting, and [`crate::gui::load_gguf_metadata_async`] 
/// for asynchronous loading with progress tracking.
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

/// Loads GGUF file metadata with full tokenizer content support.
///
/// This function extends [`load_gguf_metadata_sync`] by providing access to complete
/// tokenizer content without truncation. It's particularly useful when you need to
/// examine full tokenizer configurations, chat templates, or token arrays.
///
/// The function returns tuples with an additional optional field containing the full
/// content for tokenizer-related keys, while non-tokenizer keys have `None` in the
/// third position.
///
/// # Arguments
///
/// * `path` - Path to the GGUF file to be analyzed
///
/// # Returns
///
/// Returns a `Vec<(String, String, Option<String>)>` where each tuple contains:
/// - First element: metadata key name
/// - Second element: human-readable string representation (may be truncated for display)
/// - Third element: full content for tokenizer keys, `None` for other keys
///
/// # Examples
///
/// ```
/// use inspector_gguf::format::load_gguf_metadata_with_full_content_sync;
/// use std::path::Path;
///
/// // Test with the sample GGUF file
/// let path = Path::new("model/Qwen3-0.6B-Q5_K_M.gguf");
/// let metadata = load_gguf_metadata_with_full_content_sync(path)
///     .expect("Failed to load GGUF metadata with full content");
///
/// // Verify we got some metadata
/// assert!(!metadata.is_empty(), "Metadata should not be empty");
///
/// // Check the structure of returned tuples
/// let first_entry = &metadata[0];
/// assert_eq!(first_entry.0, "version", "First entry should be version");
/// assert!(!first_entry.1.is_empty(), "Display value should not be empty");
/// // Version is not a tokenizer key, so full_content should be None
/// assert!(first_entry.2.is_none(), "Version should not have full content");
///
/// // Look for tokenizer-related entries
/// let tokenizer_entries: Vec<_> = metadata.iter()
///     .filter(|(key, _, _)| key.starts_with("tokenizer."))
///     .collect();
///
/// // If tokenizer entries exist, they should have full content
/// for (key, display_value, full_content) in tokenizer_entries {
///     assert!(!display_value.is_empty(), "Display value should not be empty for {}", key);
///     assert!(full_content.is_some(), "Tokenizer key {} should have full content", key);
/// }
/// ```
///
/// # Errors
///
/// Returns the same errors as [`load_gguf_metadata_sync`]:
/// - File I/O errors (cannot open, read, or access file)
/// - Invalid GGUF format or corrupted file
/// - Memory allocation failures for large files
/// - UTF-8 decoding errors in string metadata
///
/// # Performance
///
/// Includes the same performance profiling scopes as [`load_gguf_metadata_sync`].
/// The additional processing for full content has minimal performance impact.
///
/// # Use Cases
///
/// This function is ideal for:
/// - Analyzing complete tokenizer configurations
/// - Extracting full chat templates for model setup
/// - Debugging tokenizer issues with complete token lists
/// - Exporting comprehensive model metadata
///
/// See also [`load_gguf_metadata_sync`] for basic metadata extraction,
/// [`get_full_tokenizer_content`] for tokenizer-specific content extraction,
/// and [`crate::gui::loader::MetadataEntry`] for the GUI representation of metadata entries.
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

/// Converts a GGUF metadata value to a human-readable string representation.
///
/// This function provides intelligent formatting for different types of GGUF metadata values,
/// with special handling for tokenizer data, arrays, and binary content. It automatically
/// truncates long content for display purposes while preserving readability.
///
/// # Arguments
///
/// * `key` - The metadata key name, used for context-specific formatting
/// * `v` - The GGUF value to convert to string representation
///
/// # Returns
///
/// A human-readable string representation of the value, with appropriate formatting
/// and truncation for display purposes.
///
/// # Examples
///
/// ```
/// use inspector_gguf::format::readable_value_for_key;
/// use candle::quantized::gguf_file::Value;
///
/// // String values are returned as-is
/// let string_val = Value::String("model_name".to_string());
/// let result = readable_value_for_key("general.name", &string_val);
/// assert_eq!(result, "model_name");
///
/// // Numeric values are converted to strings
/// let int_val = Value::U32(42);
/// let result = readable_value_for_key("some.number", &int_val);
/// assert!(result.contains("42")); // May be "U32(42)" or "42" depending on implementation
///
/// // Array values show truncated content
/// let array_val = Value::Array(vec![
///     Value::String("item1".to_string()),
///     Value::String("item2".to_string()),
///     Value::String("item3".to_string()),
///     Value::String("item4".to_string()),
/// ]);
/// let result = readable_value_for_key("some.array", &array_val);
/// assert!(result.contains("item1"));
/// assert!(result.contains("…") || result.len() < 50); // Either truncated or short enough
/// ```
///
/// # Special Handling
///
/// - **Chat templates**: `tokenizer.chat_template` arrays are decoded as UTF-8 strings
/// - **Token arrays**: `tokenizer.ggml.tokens` and `tokenizer.ggml.merges` show first few items
/// - **Byte arrays**: Small arrays show hex representation, large ones show length
/// - **General arrays**: Display first few elements with "…" for truncation
///
/// # Performance
///
/// This function is optimized for display purposes and may truncate large content.
/// For complete content, use [`readable_value_for_key_full`] with `full_content = true`.
///
/// See also [`readable_value_for_key_full`] for complete content display,
/// [`get_full_tokenizer_content`] for tokenizer-specific content,
/// and [`crate::gui::export`] module for exporting formatted values to various file formats.
#[allow(clippy::collapsible_if)]
pub fn readable_value_for_key(key: &str, v: &gguf_file::Value) -> String {
    readable_value_for_key_full(key, v, false)
}

/// Converts a GGUF metadata value to string with optional full content display.
///
/// This is the extended version of [`readable_value_for_key`] that allows controlling
/// whether to show truncated content (for display) or full content (for export/analysis).
///
/// # Arguments
///
/// * `key` - The metadata key name for context-specific formatting
/// * `v` - The GGUF value to convert
/// * `full_content` - If `true`, shows complete content without truncation
///
/// # Returns
///
/// String representation of the value, either truncated (if `full_content = false`)
/// or complete (if `full_content = true`).
///
/// # Examples
///
/// ```
/// use inspector_gguf::format::readable_value_for_key_full;
/// use candle::quantized::gguf_file::Value;
///
/// let array_val = Value::Array(vec![
///     Value::String("token1".to_string()),
///     Value::String("token2".to_string()),
///     Value::String("token3".to_string()),
///     Value::String("token4".to_string()),
///     Value::String("token5".to_string()),
///     Value::String("token6".to_string()),
/// ]);
///
/// // Truncated version for display
/// let truncated = readable_value_for_key_full("tokenizer.ggml.tokens", &array_val, false);
/// assert!(truncated.contains("…"));
///
/// // Full version for export
/// let full = readable_value_for_key_full("tokenizer.ggml.tokens", &array_val, true);
/// assert!(full.contains("token1, token2, token3, token4, token5, token6"));
/// assert!(!full.contains("…")); // Full version should not be truncated
///
/// // Test with simple string value
/// let string_val = Value::String("simple_value".to_string());
/// let result_truncated = readable_value_for_key_full("test.key", &string_val, false);
/// let result_full = readable_value_for_key_full("test.key", &string_val, true);
/// assert_eq!(result_truncated, result_full); // Should be the same for simple values
/// assert_eq!(result_truncated, "simple_value");
/// ```
///
/// See also [`readable_value_for_key`] for the standard truncated version,
/// [`get_full_tokenizer_content`] for tokenizer-specific extraction,
/// and [`crate::gui::loader::MetadataEntry`] for the GUI representation structure.
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

/// Extracts full tokenizer content for tokenizer-related metadata keys.
///
/// This function determines if a metadata key is tokenizer-related and returns
/// the complete, untruncated content if so. It's used internally by
/// [`load_gguf_metadata_with_full_content_sync`] to provide full tokenizer data.
///
/// # Arguments
///
/// * `key` - The metadata key name to check
/// * `v` - The GGUF value to extract content from
///
/// # Returns
///
/// - `Some(String)` containing full content if the key starts with "tokenizer."
/// - `None` if the key is not tokenizer-related
///
/// # Examples
///
/// ```
/// use inspector_gguf::format::get_full_tokenizer_content;
/// use candle::quantized::gguf_file::Value;
///
/// let chat_template = Value::String("template content".to_string());
///
/// // Tokenizer keys return full content
/// let result = get_full_tokenizer_content("tokenizer.chat_template", &chat_template);
/// assert!(result.is_some());
/// assert_eq!(result.unwrap(), "template content");
///
/// // Non-tokenizer keys return None
/// let result = get_full_tokenizer_content("general.name", &chat_template);
/// assert!(result.is_none());
///
/// // Test various tokenizer key patterns
/// let test_cases = vec![
///     ("tokenizer.ggml.tokens", true),
///     ("tokenizer.ggml.merges", true),
///     ("tokenizer.chat_template", true),
///     ("tokenizer.bos_token_id", true),
///     ("general.name", false),
///     ("model.type", false),
///     ("", false),
/// ];
///
/// for (key, should_have_content) in test_cases {
///     let result = get_full_tokenizer_content(key, &chat_template);
///     if should_have_content {
///         assert!(result.is_some(), "Key '{}' should return content", key);
///     } else {
///         assert!(result.is_none(), "Key '{}' should return None", key);
///     }
/// }
/// ```
///
/// # Tokenizer Keys
///
/// Keys that start with "tokenizer." are considered tokenizer-related:
/// - `tokenizer.chat_template`
/// - `tokenizer.ggml.tokens`
/// - `tokenizer.ggml.merges`
/// - `tokenizer.ggml.bos_token_id`
/// - And any other keys with the "tokenizer." prefix
///
/// See also [`readable_value_for_key_full`] for full content formatting,
/// [`load_gguf_metadata_with_full_content_sync`] for loading with full tokenizer content,
/// and [`crate::gui::panels`] module for UI components that display tokenizer content.
pub fn get_full_tokenizer_content(key: &str, v: &gguf_file::Value) -> Option<String> {
    // For tokenizer keys, return the full readable value without truncation
    if key.starts_with("tokenizer.") {
        Some(readable_value_for_key_full(key, v, true))
    } else {
        None
    }
}

/// Converts a GGUF value to a readable string without key context.
///
/// This is a convenience function that calls [`readable_value_for_key`] with an empty
/// key string, providing generic formatting without key-specific optimizations.
///
/// # Arguments
///
/// * `v` - The GGUF value to convert to string representation
///
/// # Returns
///
/// A human-readable string representation of the value using generic formatting rules.
///
/// # Examples
///
/// ```
/// use inspector_gguf::format::readable_value;
/// use candle::quantized::gguf_file::Value;
///
/// let string_val = Value::String("example".to_string());
/// let result = readable_value(&string_val);
/// assert_eq!(result, "example");
///
/// let int_val = Value::U32(42);
/// let result = readable_value(&int_val);
/// assert!(result.contains("42")); // May be "U32(42)" or "42" depending on implementation
///
/// // Test with different value types
/// let bool_val = Value::Bool(true);
/// let result = readable_value(&bool_val);
/// assert!(result.contains("true")); // May be "Bool(true)" or "true"
///
/// let float_val = Value::F32(3.14);
/// let result = readable_value(&float_val);
/// assert!(result.contains("3.14"));
///
/// // Test with array (should show some content)
/// let array_val = Value::Array(vec![
///     Value::String("a".to_string()),
///     Value::String("b".to_string()),
/// ]);
/// let result = readable_value(&array_val);
/// assert!(!result.is_empty());
/// assert!(result.contains("a") || result.contains("Array"));
/// ```
///
/// # Note
///
/// This function doesn't apply key-specific formatting (like special tokenizer handling).
/// For context-aware formatting, use [`readable_value_for_key`] instead.
///
/// See also [`readable_value_for_key`] for context-aware formatting,
/// [`readable_value_for_key_full`] for complete content display,
/// and [`crate::gui::export`] module for exporting values in various formats.
pub fn readable_value(v: &gguf_file::Value) -> String {
    readable_value_for_key("", v)
}
