// Generated macro for impl_392 (impl)
macro_rules! Depcrate_config_file_linesimpl_392 {
() => {
// Module: crate::config::file_lines
// Provides: {"impl_392"}
// Dependencies: {}
impl < 'de > :: serde :: de :: Deserialize < 'de > for FileLines { fn deserialize < D > (_ : D) -> Result < Self , D :: Error > where D : :: serde :: de :: Deserializer < 'de > , { panic ! ("FileLines cannot be deserialized from a project rustfmt.toml file: please \
             specify it via the `--file-lines` option instead") ; } }
};
}
