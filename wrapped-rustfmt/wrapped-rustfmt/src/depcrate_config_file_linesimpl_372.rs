// Generated macro for impl_372 (impl)
macro_rules! Depcrate_config_file_linesimpl_372 {
() => {
// Module: crate::config::file_lines
// Provides: {"impl_372"}
// Dependencies: {}
impl From < rustc_span :: FileName > for FileName { fn from (name : rustc_span :: FileName) -> FileName { match name { rustc_span :: FileName :: Real (rustc_span :: RealFileName :: LocalPath (p)) => FileName :: Real (p) , rustc_span :: FileName :: Custom (ref f) if f == "stdin" => FileName :: Stdin , _ => unreachable ! () , } } }
};
}
