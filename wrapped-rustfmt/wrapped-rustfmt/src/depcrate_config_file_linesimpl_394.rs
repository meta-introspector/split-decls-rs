// Generated macro for impl_394 (impl)
macro_rules! Depcrate_config_file_linesimpl_394 {
() => {
// Module: crate::config::file_lines
// Provides: {"impl_394"}
// Dependencies: {}
impl < 'de > :: serde :: de :: Deserialize < 'de > for FileLines { fn deserialize < D > (_ : D) -> Result < Self , D :: Error > where D : :: serde :: de :: Deserializer < 'de > , { panic ! ("FileLines cannot be deserialized from a project rustfmt.toml file: please \
             specify it via the `--file-lines` option instead") ; } }
};
}
