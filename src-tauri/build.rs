fn main() {
    // Der Build-Marker (release, local-<hash>, ci) steckt zur Compile-Zeit in der App.
    println!("cargo:rerun-if-env-changed=LOTSE_BUILD");
    tauri_build::build()
}
