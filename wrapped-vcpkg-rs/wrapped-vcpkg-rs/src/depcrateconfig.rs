// Generated macro for Config (struct)
macro_rules! DepcrateConfig {
() => {
// Module: crate
// Provides: {"Config"}
// Dependencies: {}
# [doc = " Configuration options for finding packages, setting up the tree and emitting metadata to cargo"] # [derive (Default)] pub struct Config { # [doc = " should the cargo metadata actually be emitted"] cargo_metadata : bool , # [doc = " should cargo:include= metadata be emitted (defaults to false)"] emit_includes : bool , # [doc = " .lib/.a files that must be be found for probing to be considered successful"] required_libs : Vec < String > , # [doc = " .dlls that must be be found for probing to be considered successful"] required_dlls : Vec < String > , # [doc = " should DLLs be copied to OUT_DIR?"] copy_dlls : bool , # [doc = " override vcpkg installed path, regardless of both VCPKG_ROOT/installed and VCPKG_INSTALLED_ROOT environment variables"] vcpkg_installed_root : Option < PathBuf > , # [doc = " override VCPKG_ROOT environment variable"] vcpkg_root : Option < PathBuf > , target : Option < TargetTriplet > , }
};
}
