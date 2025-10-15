use inspector_gguf::gui;
use structopt::StructOpt;

use std::path::PathBuf;
use image::GenericImageView;
use egui::IconData;


#[cfg(target_os = "windows")]
fn set_console_title(title: &str) {
    use winapi::um::wincon::SetConsoleTitleA;
    use std::ffi::CString;

    let c_title = CString::new(title).unwrap();
    unsafe {
        SetConsoleTitleA(c_title.as_ptr());
    }
}

#[cfg(not(target_os = "windows"))]
fn set_console_title(_title: &str) {
    // На других платформах ничего не делаем
}

fn load_icon() -> Result<IconData, Box<dyn std::error::Error>> {
    // Используем высококачественную иконку большого размера для лучшей видимости
    let img = image::open("assets/icons/128x128@2x.png")?;
    let (width, height) = img.dimensions();
    let rgba = img.to_rgba8().into_raw();
    Ok(IconData {
        rgba,
        width,
        height,
    })
}

#[derive(StructOpt, Debug)]
#[structopt(name = "gguf-inspector")]
struct Opt {
    /// Run GUI application
    #[structopt(long)]
    gui: bool,

    /// Run profiling test with real model file
    #[structopt(long)]
    profile: bool,

    /// Directory with pre-extracted metadata YAML files to validate
    #[structopt(long, parse(from_os_str))]
    metadata_dir: Option<PathBuf>,

    /// Path to GGUF file for CLI export
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,

    /// Output JSON file (CLI only)
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    // Устанавливаем заголовок консольного окна
    set_console_title("Inspector GGUF");

    // Initialize puffin profiler and server only when profiling
    let _puffin_server = if opt.profile {
        // Initialize puffin profiler
        puffin::set_scopes_on(true);

        // Start puffin_http server for web-based profiling
        let server = puffin_http::Server::new("127.0.0.1:8585").unwrap();
        println!("Puffin profiler server started on http://127.0.0.1:8585");
        Some(server)
    } else {
        None
    };

    // Profiling test mode with real model file
    if opt.profile {
        let model_path = std::path::PathBuf::from("model/Qwen3-0.6B-Q5_K_M.gguf");
        if !model_path.exists() {
            eprintln!("Model file not found: {}", model_path.display());
            return Err("Model file not found".into());
        }

        println!(
            "Starting profiling with real model file: {}",
            model_path.display()
        );

        // Initialize system monitor
        let mut system = sysinfo::System::new_all();
        system.refresh_all();

        // Capture initial system state
        let initial_memory = system.used_memory();
        let initial_cpu = system.global_cpu_info().cpu_usage();

        // Start timing
        let profiling_start = std::time::Instant::now();

        puffin::profile_scope!("profiling_test");

        // File reading phase
        let file_read_start = std::time::Instant::now();
        let file_size = match std::fs::metadata(&model_path) {
            Ok(metadata) => metadata.len(),
            Err(_) => 0,
        };

        let mut f = std::fs::File::open(&model_path)?;
        let mut buf = Vec::new();
        use std::io::Read;
        f.read_to_end(&mut buf)?;
        let file_read_duration = file_read_start.elapsed();

        // GGUF parsing phase
        let parsing_start = std::time::Instant::now();
        let mut cursor = std::io::Cursor::new(&buf);
        let _content = candle::quantized::gguf_file::Content::read(&mut cursor)?;
        let parsing_duration = parsing_start.elapsed();

        // Metadata processing phase
        let metadata_start = std::time::Instant::now();
        let metadata_result = inspector_gguf::format::load_gguf_metadata_with_full_content_sync(&model_path);
        let metadata_duration = metadata_start.elapsed();

        let total_duration = profiling_start.elapsed();

        // Capture final system state
        system.refresh_all();
        let final_memory = system.used_memory();
        let final_cpu = system.global_cpu_info().cpu_usage();

        // Calculate memory usage (approximate)
        let memory_used_kb = final_memory.saturating_sub(initial_memory);

        // Save profiling results and metadata to file
        let profiling_results = match &metadata_result {
            Ok(metadata) => {
                println!(
                    "Successfully loaded {} metadata entries from real model",
                    metadata.len()
                );
                // Print some sample metadata
                for (key, value, _) in metadata.iter().take(5) {
                    println!("  {}: {}", key, value.chars().take(50).collect::<String>());
                }

                // Create profiling report with performance metrics
                let sample_metadata: std::collections::HashMap<String, String> = metadata
                    .iter()
                    .take(10)
                    .map(|(k, v, _)| (k.clone(), v.clone()))
                    .collect();

                serde_json::json!({
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "model_path": model_path.display().to_string(),
                    "model_info": {
                        "file_size_bytes": file_size,
                        "file_size_mb": file_size as f64 / (1024.0 * 1024.0),
                        "metadata_count": metadata.len()
                    },
                    "performance_metrics": {
                        "total_duration_ms": total_duration.as_millis(),
                        "total_duration_secs": total_duration.as_secs_f64(),
                        "file_read_duration_ms": file_read_duration.as_millis(),
                        "gguf_parsing_duration_ms": parsing_duration.as_millis(),
                        "metadata_processing_duration_ms": metadata_duration.as_millis(),
                        "memory_used_kb": memory_used_kb,
                        "initial_memory_kb": initial_memory,
                        "final_memory_kb": final_memory,
                        "cpu_usage_initial": initial_cpu,
                        "cpu_usage_final": final_cpu,
                        "throughput_mb_per_sec": (file_size as f64 / (1024.0 * 1024.0)) / total_duration.as_secs_f64()
                    },
                    "sample_metadata": sample_metadata,
                    "status": "success"
                })
            }
            Err(e) => {
                eprintln!("Failed to load model file: {}", e);
                serde_json::json!({
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "model_path": model_path.display().to_string(),
                    "performance_metrics": {
                        "total_duration_ms": total_duration.as_millis(),
                        "memory_used_kb": memory_used_kb,
                        "error_occurred": true
                    },
                    "status": "error",
                    "error": e.to_string()
                })
            }
        };

        // Save to file
        let report_path = std::path::PathBuf::from("profile.json");
        if let Err(e) = std::fs::write(
            &report_path,
            serde_json::to_string_pretty(&profiling_results).unwrap(),
        ) {
            eprintln!("Failed to save profiling report: {}", e);
        } else {
            println!("Profiling report saved to: {}", report_path.display());
        }

        // Mark frame to ensure all profiling data is collected
        puffin::GlobalProfiler::lock().new_frame();

        println!("Profiling test completed");
        println!("Server is still running at http://127.0.0.1:8585");
        println!("You can now open the URL in your browser to view the profiling results");
        println!("Press Ctrl+C to stop the server and exit");

        // Server continues running in background - user can stop with Ctrl+C when done

        metadata_result.map(|_| ())?
    }

    if opt.gui {
        let icon = load_icon().unwrap_or_else(|_| {
            eprintln!("Warning: Failed to load icon, using default");
            IconData::default()
        });

        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([960.0, 600.0])
                .with_min_inner_size([640.0, 360.0])
                .with_decorations(true)
                .with_transparent(false) // Disable transparency to avoid potential issues
                .with_icon(icon),
            ..Default::default()
        };
        
        let _ = eframe::run_native(
            "Inspector GGUF",
            native_options,
            Box::new(|_cc| Ok(Box::new(gui::GgufApp::default()))),
        );
        return Ok(());
    }

    // Если не указаны входные аргументы, по умолчанию проверим каталог GGUF в корне проекта
    if opt.input.is_none() && opt.metadata_dir.is_none() {
        // Try to detect repository root by looking for README.md or .git at current or parent directories
        let mut cwd = std::env::current_dir()?;
        let mut repo_root: Option<PathBuf> = None;
        for _ in 0..6 {
            if cwd.join("README.md").exists() || cwd.join(".git").exists() {
                repo_root = Some(cwd.clone());
                break;
            }
            if !cwd.pop() {
                break;
            }
        }
        if let Some(root) = repo_root {
            let default_gguf_dir = root.join("models/gguf");
            if default_gguf_dir.exists() {
                check_gguf_dir(&default_gguf_dir)?;
                return Ok(());
            }
        }
    }

    // CLI mode: если указана директория с YAML метаданными — проверим её
    if let Some(ref dir) = opt.metadata_dir {
        check_metadata_dir(dir)?;
        return Ok(());
    }

    // CLI mode: fallback to previous behavior if input provided
    if let Some(input) = opt.input {
        // Use our improved metadata loading function
        let metadata = inspector_gguf::format::load_gguf_metadata_with_full_content_sync(&input)?;

        let mut map = serde_json::Map::new();
        let mut keys = Vec::new();

        for (k, v, _) in &metadata {
            keys.push(k.clone());
            // Try to parse as JSON, fallback to string
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(v) {
                map.insert(k.clone(), json);
            } else {
                map.insert(k.clone(), serde_json::Value::String(v.clone()));
            }
        }

        let exported = serde_json::json!({"keys": keys, "raw": serde_json::Value::Object(map)});
        let out_path = match opt.output {
            Some(p) => p,
            None => input.with_extension("gguf.metadata.json"),
        };
        std::fs::write(out_path, serde_json::to_string_pretty(&exported)?)?;
        println!("OK");
    } else {
        eprintln!("No input provided. Use --gui to run the GUI or provide input path for CLI.");
    }
    Ok(())
}

fn check_metadata_dir(dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !dir.is_dir() {
        return Err(format!("Not a directory: {}", dir.display()).into());
    }
    for entry in std::fs::read_dir(dir)? {
        let ent = entry?;
        let path = ent.path();
        if let Some(ext) = path.extension().and_then(|s| s.to_str())
            && (ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml"))
        {
            println!("Checking {}...", path.display());
            let bytes = std::fs::read(&path)?;
            let yaml: serde_yaml::Value = serde_yaml::from_slice(&bytes)?;
            // Flatten to map of strings for basic validation
            if let Some(map) = yaml.as_mapping() {
                let mut has_tokenizer = false;
                let mut has_config = false;
                for (k, _v) in map.iter() {
                    if let Some(kstr) = k.as_str() {
                        if kstr.contains("tokenizer") {
                            has_tokenizer = true;
                        }
                        if kstr.contains("config") {
                            has_config = true;
                        }
                    }
                }
                println!(
                    "  keys: {} entries, tokenizer_in_metadata={}, config_in_metadata={}",
                    map.len(),
                    has_tokenizer,
                    has_config
                );
            } else {
                println!("  not a mapping — skipping");
            }
        }
    }
    Ok(())
}

fn check_gguf_dir(dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !dir.is_dir() {
        return Err(format!("Not a directory: {}", dir.display()).into());
    }
    for entry in std::fs::read_dir(dir)? {
        let ent = entry?;
        let path = ent.path();
        if path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("gguf"))
            .unwrap_or(false)
        {
            println!("Reading {}...", path.display());
            let mut f = std::fs::File::open(&path)?;
            use std::io::Read;
            let mut buf = Vec::new();
            f.read_to_end(&mut buf)?;
            let mut cursor = std::io::Cursor::new(&buf);
            let content = candle::quantized::gguf_file::Content::read(&mut cursor)?;
            let keys: Vec<String> = content.metadata.keys().cloned().collect();
            let _joined = keys.join(", ");
            // Проверяем наличие токенизатора/конфига в метаданных
            let has_tokenizer = keys.iter().any(|k| {
                k.contains("tokenizer")
                    || k.contains("tokenizer.json")
                    || k.contains("tokenizer.ggml")
            });
            let has_config = keys.iter().any(|k| {
                k.contains("config")
                    || k.contains("config.json")
                    || k.contains("general.config_json")
            });
            println!(
                "  keys_count={}, tokenizer_in_metadata={}, config_in_metadata={}",
                keys.len(),
                has_tokenizer,
                has_config
            );
            if !has_tokenizer {
                println!(
                    "  WARNING: tokenizer not found in GGUF metadata for {}",
                    path.display()
                );
            }
            if !has_config {
                println!(
                    "  WARNING: config not found in GGUF metadata for {}",
                    path.display()
                );
            }
        }
    }
    Ok(())
}
