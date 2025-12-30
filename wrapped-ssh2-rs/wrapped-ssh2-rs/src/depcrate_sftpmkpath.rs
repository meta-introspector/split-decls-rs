// Generated macro for mkpath (function)
macro_rules! Depcrate_sftpmkpath {
() => {
// Module: crate::sftp
// Provides: {"mkpath"}
// Dependencies: {}
# [cfg (windows)] fn mkpath (v : Vec < u8 >) -> PathBuf { use std :: str ; PathBuf :: from (str :: from_utf8 (& v) . unwrap ()) }
};
}
