fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let mut res = winres::WindowsResource::new();
        
        // Set icon if it exists
        if std::path::Path::new("assets/icon.ico").exists() {
            res.set_icon("assets/icon.ico");
        } else {
            println!("cargo:warning=Icon file 'assets/icon.ico' not found. Executable will use default icon.");
            println!("cargo:warning=To add an icon, place an 'icon.ico' file in the 'assets/' directory.");
        }
        
        res.compile().unwrap();
    }
}

