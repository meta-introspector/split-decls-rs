// Generated macro for find_vcpkg_root (function)
macro_rules! Depcratefind_vcpkg_root {
() => {
// Module: crate
// Provides: {"find_vcpkg_root"}
// Dependencies: {}
# [doc = " Find the vcpkg root"] # [doc (hidden)] pub fn find_vcpkg_root (cfg : & Config) -> Result < PathBuf , Error > { if let & Some (ref path) = & cfg . vcpkg_root { return Ok (path . clone ()) ; } if let Some (path) = env :: var_os ("VCPKG_ROOT") { return Ok (PathBuf :: from (path)) ; } if let Ok (ref local_app_data) = env :: var ("LOCALAPPDATA") { let vcpkg_user_targets_path = Path :: new (local_app_data . as_str ()) . join ("vcpkg") . join ("vcpkg.user.targets") ; if let Ok (file) = File :: open (vcpkg_user_targets_path . clone ()) { let file = BufReader :: new (& file) ; for line in file . lines () { let line = try ! (line . map_err (| _ | Error :: VcpkgNotFound (format ! ("Parsing of {} failed." , vcpkg_user_targets_path . to_string_lossy () . to_owned ())))) ; let mut split = line . split ("Project=\"") ; split . next () ; if let Some (found) = split . next () { if let Some (found) = found . split_terminator ('"') . next () { let mut vcpkg_root = PathBuf :: from (found) ; if ! (vcpkg_root . pop () && vcpkg_root . pop () && vcpkg_root . pop () && vcpkg_root . pop ()) { return Err (Error :: VcpkgNotFound (format ! ("Could not find vcpkg root above {}" , found))) ; } return Ok (vcpkg_root) ; } } } } } if let Some (path) = env :: var_os ("OUT_DIR") { let mut path = PathBuf :: from (path) ; while path . pop () { let mut try_root = path . clone () ; try_root . push ("vcpkg") ; try_root . push (".vcpkg-root") ; if try_root . exists () { try_root . pop () ; let mut cv_cfg = try_root . clone () ; cv_cfg . push ("downloads") ; cv_cfg . push ("cargo-vcpkg.toml") ; if cv_cfg . exists () { return Ok (try_root) ; } } } } Err (Error :: VcpkgNotFound ("No vcpkg installation found. Set the VCPKG_ROOT environment \
             variable or run 'vcpkg integrate install'" . to_string () ,)) }
};
}
