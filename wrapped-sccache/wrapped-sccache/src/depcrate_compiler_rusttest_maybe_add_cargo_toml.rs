// Generated macro for test_maybe_add_cargo_toml (function)
macro_rules! Depcrate_compiler_rusttest_maybe_add_cargo_toml {
() => {
// Module: crate::compiler::rust
// Provides: {"test_maybe_add_cargo_toml"}
// Dependencies: {}
# [test] # [cfg (feature = "dist-client")] fn test_maybe_add_cargo_toml () { let (root , result_cargo_toml_path) = if cfg ! (windows) { (r"C:\mozilla-source\mozilla-unified\third_party\rust" , r"C:\mozilla-source\mozilla-unified\third_party\rust\wgpu-core\Cargo.toml" ,) } else { ("/home/user/mozilla-source/mozilla-unified/third_party/rust" , "/home/user/mozilla-source/mozilla-unified/third_party/rust/wgpu-core/Cargo.toml" ,) } ; let wgpu_core = PathBuf :: from (& root) . join ("wgpu-core") . join ("src") . join ("core.rs") ; let wgpu_lib = PathBuf :: from (& root) . join ("wgpu-core") . join ("src") . join ("lib.rs") ; assert ! (maybe_add_cargo_toml (& wgpu_core , false) . is_none ()) ; assert ! (maybe_add_cargo_toml (& wgpu_core , true) . is_none ()) ; assert ! (maybe_add_cargo_toml (& wgpu_lib , false) == Some (PathBuf :: from (& root) . join ("wgpu-core") . join ("Cargo.toml"))) ; assert ! (maybe_add_cargo_toml (& wgpu_lib , false) . unwrap () . to_str () == Some (result_cargo_toml_path)) ; assert ! (maybe_add_cargo_toml (& wgpu_lib , true) . is_none ()) ; }
};
}
