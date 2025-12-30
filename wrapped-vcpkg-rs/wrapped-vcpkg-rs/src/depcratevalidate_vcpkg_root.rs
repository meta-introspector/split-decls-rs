// Generated macro for validate_vcpkg_root (function)
macro_rules! Depcratevalidate_vcpkg_root {
() => {
// Module: crate
// Provides: {"validate_vcpkg_root"}
// Dependencies: {}
fn validate_vcpkg_root (path : & PathBuf) -> Result < () , Error > { let mut vcpkg_root_path = path . clone () ; vcpkg_root_path . push (".vcpkg-root") ; if vcpkg_root_path . exists () { Ok (()) } else { Err (Error :: VcpkgNotFound (format ! ("Could not find Vcpkg root at {}" , vcpkg_root_path . to_string_lossy ()))) } }
};
}
