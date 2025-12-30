// Generated macro for temp_base (function)
macro_rules! Depcrate_gztemp_base {
() => {
// Module: crate::gz
// Provides: {"temp_base"}
// Dependencies: {}
fn temp_base () -> PathBuf { if cfg ! (target_os = "wasi") { std :: path :: PathBuf :: from ("/tmp/") } else { std :: env :: temp_dir () } }
};
}
