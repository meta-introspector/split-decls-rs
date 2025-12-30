// Generated macro for find_vcpkg_target (function)
macro_rules! Depcratefind_vcpkg_target {
() => {
// Module: crate
// Provides: {"find_vcpkg_target"}
// Dependencies: {}
fn find_vcpkg_target (cfg : & Config , target_triplet : & TargetTriplet) -> Result < VcpkgTarget , Error > { let vcpkg_root = try ! (find_vcpkg_root (& cfg)) ; try ! (validate_vcpkg_root (& vcpkg_root)) ; let mut base = cfg . vcpkg_installed_root . clone () . or (env :: var_os ("VCPKG_INSTALLED_ROOT") . map (PathBuf :: from)) . unwrap_or (vcpkg_root . join ("installed")) ; let status_path = base . join ("vcpkg") ; base . push (& target_triplet . triplet) ; let lib_path = base . join ("lib") ; let bin_path = base . join ("bin") ; let include_path = base . join ("include") ; let packages_path = vcpkg_root . join ("packages") ; Ok (VcpkgTarget { lib_path : lib_path , bin_path : bin_path , include_path : include_path , status_path : status_path , packages_path : packages_path , target_triplet : target_triplet . clone () , }) }
};
}
