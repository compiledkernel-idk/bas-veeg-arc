#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("app.ico");
    res.set("ProductName", "Bas Veeg Arc");
    res.set("FileDescription", "Bas Veeg Arc - School Fighting Game");
    res.set("CompanyName", "BAS VEEG ARC");
    res.set("LegalCopyright", "Copyright © 2025");
    res.compile().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=app.ico");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
}
