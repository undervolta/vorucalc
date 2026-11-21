
use std::path::PathBuf;

fn main() {
    let manifest_dir =
        PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());

    // Slint compilation
    let library_paths = std::collections::HashMap::from([
        ("styles".to_string(), manifest_dir.join("ui/styles/index.slint")),
        ("components".to_string(), manifest_dir.join("ui/components")),
        ("stores".to_string(), manifest_dir.join("ui/stores")),
    ]);

    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(library_paths);

    slint_build::compile_with_config("ui/app-window.slint", config)
        .expect("Slint build failed");

    // Windows executable icon
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let icon_path = manifest_dir.join("assets/icons/vorucalc.ico");

        println!("cargo:rerun-if-changed={}", icon_path.display());

        let mut res = winresource::WindowsResource::new();
        res.set_icon(icon_path.to_str().unwrap());

        res.compile()
            .expect("Failed to compile Windows resources");
    }
}
