fn main() {
    slint_build::compile("ui/resize.slint").unwrap();
    slint_build::compile("ui/titlebar.slint").unwrap();
}

// // Set the linker argument to use the Windows subsystem
// println!("cargo:rustc-link-arg=-Wl,--subsystem,windows");
