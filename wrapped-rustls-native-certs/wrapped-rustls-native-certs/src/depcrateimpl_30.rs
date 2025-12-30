// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl CertPaths { fn from_env () -> Self { Self { file : env :: var_os (ENV_CERT_FILE) . map (PathBuf :: from) , dirs : match env :: var_os (ENV_CERT_DIR) { Some (dirs) => env :: split_paths (& dirs) . collect () , None => Vec :: new () , } , } } # [doc = " Load certificates from the paths."] # [doc = ""] # [doc = " See [`load_certs_from_paths()`]."] fn load (& self) -> CertificateResult { load_certs_from_paths_internal (self . file . as_deref () , & self . dirs) } }
};
}
