// Generated macro for read_file (function)
macro_rules! Depcrate_settings_configread_file {
() => {
// Module: crate::settings::config
// Provides: {"read_file"}
// Dependencies: {}
# [cfg (feature = "rpk")] fn read_file (path : & str) -> QuicResult < Vec < u8 > > { use anyhow :: Context as _ ; std :: fs :: read (path) . with_context (| | format ! ("read {path}")) . map_err (Into :: into) }
};
}
