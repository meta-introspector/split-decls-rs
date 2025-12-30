// Generated macro for impl_377 (impl)
macro_rules! Depcrate_config_file_linesimpl_377 {
() => {
// Module: crate::config::file_lines
// Provides: {"impl_377"}
// Dependencies: {}
impl Serialize for FileName { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let s = match self { FileName :: Stdin => Ok ("stdin") , FileName :: Real (path) => path . to_str () . ok_or_else (| | ser :: Error :: custom ("path can't be serialized as UTF-8 string")) , } ; s . and_then (| s | serializer . serialize_str (s)) } }
};
}
