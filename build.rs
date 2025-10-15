#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set("FileDescription", "Inspector GGUF");
    res.set("ProductName", "Inspector GGUF");
    res.set("CompanyName", "FerrisMind");
    res.set("FileVersion", env!("CARGO_PKG_VERSION"));
    res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
    res.set_icon("assets/icons/icon_new.ico");

    // Устанавливаем Windows subsystem для GUI приложения (без консольного окна)
    res.set("Subsystem", "WINDOWS");

    if let Err(e) = res.compile() {
        eprintln!("Failed to compile resources: {}", e);
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // На других платформах ничего не делаем
}
