// Generated macro for impl_376 (impl)
macro_rules! Depcrate_config_file_linesimpl_376 {
() => {
// Module: crate::config::file_lines
// Provides: {"impl_376"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for FileName { fn deserialize < D > (deserializer : D) -> Result < FileName , D :: Error > where D : Deserializer < 'de > , { let s = String :: deserialize (deserializer) ? ; if s == "stdin" { Ok (FileName :: Stdin) } else { Ok (FileName :: Real (s . into ())) } } }
};
}
