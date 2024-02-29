use std::env;
use std::process::Command;

fn main() {
    let lib_dir = env::var("OUT_DIR").unwrap();
    dbg!(&lib_dir);

    // Assuming the library is in the project root
    let go_library_url = "https://github.com/clundin25/scalable-auth-core.git";

    // Download (consider using reqwest for more robust downloading)
    Command::new("git")
        .args(&["clone", &go_library_url, "auth-core"])
        .current_dir(&lib_dir)
        .status()
        .unwrap();

    // Build the Go library
    let go_library_root = format!("{}/auth-core", &lib_dir);
    Command::new("./build.sh")
        .current_dir(&go_library_root)
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", &go_library_root);
    println!("cargo:rustc-link-lib=static=scalable-auth");
}
