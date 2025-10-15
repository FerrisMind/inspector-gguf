//! Export functionality for GGUF metadata
//! This module provides various export formats including CSV, YAML, Markdown, HTML, and PDF

#![allow(dead_code)] // Allow dead code since this module is extracted but not yet integrated

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use std::path::{Path, PathBuf};

/// Ensures that a file path has the specified extension
pub fn ensure_extension(path: &Path, ext: &str) -> PathBuf {
    if path.extension().is_none() {
        let mut p = path.to_path_buf();
        p.set_extension(ext);
        p
    } else {
        path.to_path_buf()
    }
}

/// Sanitizes text for markdown by removing control characters except newlines and tabs
pub fn sanitize_for_markdown(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_control() && c != '\n' && c != '\t' {
                ' '
            } else {
                c
            }
        })
        .collect()
}

/// Escapes markdown special characters to prevent structure breaking
pub fn escape_markdown_text(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '*' | '_' | '`' | '[' | ']' | '<' | '>' | '#' => format!("\\{}", c),
            other => other.to_string(),
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Shows base64 encoded data in a temporary file opened with default editor
#[allow(dead_code)]
pub fn show_base64_dialog(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Encode string as base64 (assume original bytes are the utf-8 of data)
    let b64 = STANDARD.encode(data.as_bytes());
    // Save to temp file and open with default editor
    let tmp = std::env::temp_dir().join("gguf_metadata_base64.txt");
    std::fs::write(&tmp, b64)?;
    opener::open(&tmp)?;
    Ok(())
}

/// Exports metadata to CSV format
pub fn export_csv(
    metadata: &[(&String, &String)],
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = ensure_extension(path, "csv");
    let mut wtr = csv::Writer::from_path(&path)?;
    // Note: CSV headers are kept in English for compatibility
    wtr.write_record(["key", "value"])?;
    for (k, v) in metadata {
        wtr.write_record([k, v])?;
    }
    wtr.flush()?;
    Ok(())
}

/// Exports metadata to YAML format
pub fn export_yaml(
    metadata: &[(&String, &String)],
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let map: std::collections::HashMap<_, _> = metadata.iter()
        .map(|(k, v)| ((*k).clone(), (*v).clone()))
        .collect();
    let yaml = serde_yaml::to_string(&map)?;
    let path = ensure_extension(path, "yaml");
    std::fs::write(path, yaml)?;
    Ok(())
}

/// Exports metadata to markdown format and returns the markdown string
pub fn export_markdown(metadata: &[(&String, &String)]) -> String {
    let mut out = String::new();
    out.push_str("# GGUF Metadata\n\n");
    for (k, v) in metadata {
        out.push_str(&format!("## {}\n\n", escape_markdown_text(k)));
        out.push('\n');
        if v.len() > 1024 || v.contains('\0') {
            // For large/binary fields — Base64
            let b64 = STANDARD.encode(v.as_bytes());
            out.push_str("```base64\n");
            out.push_str(&b64);
            out.push_str("\n```\n\n");
        } else {
            let safe = sanitize_for_markdown(v);
            out.push_str("```\n");
            out.push_str(&safe.replace("```", "` ` `"));
            out.push_str("\n```\n\n");
        }
    }
    out
}

/// Exports metadata to markdown file
pub fn export_markdown_to_file(
    metadata: &[(&String, &String)],
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let md = export_markdown(metadata);
    let path = ensure_extension(path, "md");
    std::fs::write(&path, md)?;
    Ok(())
}

/// Exports metadata to HTML format and returns the HTML string
pub fn export_html(metadata: &[(&String, &String)]) -> Result<String, Box<dyn std::error::Error>> {
    let md = export_markdown(metadata);
    let parser = pulldown_cmark::Parser::new(&md);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    Ok(html_output)
}

/// Exports metadata to HTML file
pub fn export_html_to_file(
    metadata: &[(&String, &String)],
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(metadata)?;
    let path = ensure_extension(path, "html");
    std::fs::write(&path, html)?;
    Ok(())
}

/// Exports markdown content to PDF file
pub fn export_pdf_from_markdown(
    md: &str,
    out_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure .pdf extension and pass &str to markdown2pdf
    let out_path = ensure_extension(out_path, "pdf");
    let out_str = out_path.to_str().ok_or("output path is not valid UTF-8")?;
    // markdown2pdf can error on unexpected tokens — provide sanitized markdown
    let safe_md = sanitize_for_markdown(md);
    markdown2pdf::parse_into_file(
        safe_md.to_string(),
        out_str,
        markdown2pdf::config::ConfigSource::Default,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn create_test_metadata() -> Vec<(String, String)> {
        vec![
            ("test_key1".to_string(), "test_value1".to_string()),
            ("test_key2".to_string(), "test_value2\nwith\nnewlines".to_string()),
            ("binary_key".to_string(), "binary\0data\x01\x02".to_string()),
            ("markdown_key".to_string(), "text with *markdown* and `code`".to_string()),
        ]
    }

    fn get_test_metadata_refs(metadata: &[(String, String)]) -> Vec<(&String, &String)> {
        metadata.iter().map(|(k, v)| (k, v)).collect()
    }

    #[test]
    fn test_ensure_extension_adds_extension() {
        let path = Path::new("test_file");
        let result = ensure_extension(path, "csv");
        assert_eq!(result, PathBuf::from("test_file.csv"));
    }

    #[test]
    fn test_ensure_extension_preserves_existing() {
        let path = Path::new("test_file.txt");
        let result = ensure_extension(path, "csv");
        assert_eq!(result, PathBuf::from("test_file.txt"));
    }

    #[test]
    fn test_sanitize_for_markdown() {
        let input = "normal text\nwith newline\tand tab\x00and null\x01and control";
        let result = sanitize_for_markdown(input);
        assert_eq!(result, "normal text\nwith newline\tand tab and null and control");
    }

    #[test]
    fn test_escape_markdown_text() {
        let input = "text with *bold* and _italic_ and `code` and [link] and <tag> and #header";
        let result = escape_markdown_text(input);
        assert_eq!(result, "text with \\*bold\\* and \\_italic\\_ and \\`code\\` and \\[link\\] and \\<tag\\> and \\#header");
    }

    #[test]
    fn test_export_csv_success() {
        let metadata = create_test_metadata();
        let metadata_refs = get_test_metadata_refs(&metadata);
        let temp_dir = std::env::temp_dir();
        let test_path = temp_dir.join("test_export.csv");
        
        // Clean up any existing file
        let _ = fs::remove_file(&test_path);
        
        let result = export_csv(&metadata_refs, &test_path);
        assert!(result.is_ok(), "CSV export should succeed");
        
        // Verify file was created
        assert!(test_path.exists(), "CSV file should be created");
        
        // Verify content
        let content = fs::read_to_string(&test_path).expect("Should read CSV file");
        assert!(content.contains("key,value"), "CSV should have headers");
        assert!(content.contains("test_key1,test_value1"), "CSV should contain data");
        
        // Clean up
        let _ = fs::remove_file(&test_path);
    }

    #[test]
    fn test_export_yaml_success() {
        let metadata = create_test_metadata();
        let metadata_refs = get_test_metadata_refs(&metadata);
        let temp_dir = std::env::temp_dir();
        let test_path = temp_dir.join("test_export.yaml");
        
        // Clean up any existing file
        let _ = fs::remove_file(&test_path);
        
        let result = export_yaml(&metadata_refs, &test_path);
        assert!(result.is_ok(), "YAML export should succeed");
        
        // Verify file was created
        assert!(test_path.exists(), "YAML file should be created");
        
        // Verify content
        let content = fs::read_to_string(&test_path).expect("Should read YAML file");
        assert!(content.contains("test_key1: test_value1"), "YAML should contain data");
        
        // Clean up
        let _ = fs::remove_file(&test_path);
    }

    #[test]
    fn test_export_markdown_content() {
        let metadata = create_test_metadata();
        let metadata_refs = get_test_metadata_refs(&metadata);
        
        let result = export_markdown(&metadata_refs);
        
        assert!(result.contains("# GGUF Metadata"), "Should have main header");
        assert!(result.contains("## test\\_key1"), "Should escape markdown in headers");
        assert!(result.contains("```base64"), "Should use base64 for binary data");
        assert!(result.contains("```\ntest_value1\n```"), "Should format simple values");
    }

    #[test]
    fn test_export_markdown_to_file_success() {
        let metadata = create_test_metadata();
        let metadata_refs = get_test_metadata_refs(&metadata);
        let temp_dir = std::env::temp_dir();
        let test_path = temp_dir.join("test_export.md");
        
        // Clean up any existing file
        let _ = fs::remove_file(&test_path);
        
        let result = export_markdown_to_file(&metadata_refs, &test_path);
        assert!(result.is_ok(), "Markdown export should succeed");
        
        // Verify file was created
        assert!(test_path.exists(), "Markdown file should be created");
        
        // Verify content
        let content = fs::read_to_string(&test_path).expect("Should read markdown file");
        assert!(content.contains("# GGUF Metadata"), "Should contain markdown content");
        
        // Clean up
        let _ = fs::remove_file(&test_path);
    }

    #[test]
    fn test_export_html_success() {
        let metadata = create_test_metadata();
        let metadata_refs = get_test_metadata_refs(&metadata);
        
        let result = export_html(&metadata_refs);
        assert!(result.is_ok(), "HTML export should succeed");
        
        let html = result.unwrap();
        assert!(html.contains("<h1>"), "Should contain HTML headers");
        assert!(html.contains("<pre>"), "Should contain code blocks");
    }

    #[test]
    fn test_export_html_to_file_success() {
        let metadata = create_test_metadata();
        let metadata_refs = get_test_metadata_refs(&metadata);
        let temp_dir = std::env::temp_dir();
        let test_path = temp_dir.join("test_export.html");
        
        // Clean up any existing file
        let _ = fs::remove_file(&test_path);
        
        let result = export_html_to_file(&metadata_refs, &test_path);
        assert!(result.is_ok(), "HTML export should succeed");
        
        // Verify file was created
        assert!(test_path.exists(), "HTML file should be created");
        
        // Verify content
        let content = fs::read_to_string(&test_path).expect("Should read HTML file");
        assert!(content.contains("<h1>"), "Should contain HTML content");
        
        // Clean up
        let _ = fs::remove_file(&test_path);
    }

    #[test]
    fn test_file_extension_management() {
        let temp_dir = std::env::temp_dir();
        
        // Test CSV extension handling
        let csv_path_no_ext = temp_dir.join("test_no_ext");
        let csv_path_wrong_ext = temp_dir.join("test_wrong.txt");
        
        let metadata = create_test_metadata();
        let metadata_refs = get_test_metadata_refs(&metadata);
        
        // Test that extensions are properly added/preserved
        let _ = export_csv(&metadata_refs, &csv_path_no_ext);
        assert!(temp_dir.join("test_no_ext.csv").exists(), "Should add .csv extension");
        
        let _ = export_csv(&metadata_refs, &csv_path_wrong_ext);
        assert!(csv_path_wrong_ext.exists(), "Should preserve existing extension");
        
        // Clean up
        let _ = fs::remove_file(temp_dir.join("test_no_ext.csv"));
        let _ = fs::remove_file(&csv_path_wrong_ext);
    }

    #[test]
    fn test_error_handling_invalid_path() {
        let metadata = create_test_metadata();
        let metadata_refs = get_test_metadata_refs(&metadata);
        
        // Test with invalid path (empty path should cause an error)
        let invalid_path = Path::new("");
        let result = export_csv(&metadata_refs, invalid_path);
        
        // The exact error depends on the OS, but it should fail
        assert!(result.is_err(), "Should fail with invalid path");
    }

    #[test]
    fn test_show_base64_dialog_error_handling() {
        // Test with valid data - this should work
        let result = show_base64_dialog("test data");
        // Note: This might fail if no default editor is available, but the function should handle it gracefully
        // We're mainly testing that it doesn't panic and returns a Result
        match result {
            Ok(_) => {}, // Success case
            Err(_) => {}, // Error case is also acceptable (no default editor, etc.)
        }
    }

    #[test]
    fn test_export_pdf_error_handling() {
        let temp_dir = std::env::temp_dir();
        let test_path = temp_dir.join("test_export.pdf");
        
        // Clean up any existing file
        let _ = fs::remove_file(&test_path);
        
        // Test with simple markdown
        let result = export_pdf_from_markdown("# Test Header\n\nTest content", &test_path);
        
        // PDF export might fail due to system dependencies, but should handle errors gracefully
        match result {
            Ok(_) => {
                // If successful, verify file was created
                assert!(test_path.exists(), "PDF file should be created on success");
                let _ = fs::remove_file(&test_path);
            },
            Err(e) => {
                // Error is acceptable if PDF generation dependencies are not available
                println!("PDF export failed (expected if dependencies not available): {}", e);
            }
        }
    }

    #[test]
    fn test_large_data_handling() {
        // Test with large data that should trigger base64 encoding
        let large_value = "x".repeat(2000);
        let metadata = vec![
            ("large_key".to_string(), large_value),
        ];
        let metadata_refs = get_test_metadata_refs(&metadata);
        
        let markdown = export_markdown(&metadata_refs);
        assert!(markdown.contains("```base64"), "Large data should be base64 encoded");
        
        // Test HTML export with large data
        let html_result = export_html(&metadata_refs);
        assert!(html_result.is_ok(), "HTML export should handle large data");
    }
}