// Generated macro for impl_20 (impl)
macro_rules! Depcrate_schemaimpl_20 {
() => {
// Module: crate::schema
// Provides: {"impl_20"}
// Dependencies: {}
impl OneShot { fn parse_toml (s : & str) -> Result < Self , crate :: Error > { toml_edit :: de :: from_str (s) . map_err (| e | e . to_string () . into ()) } }
};
}
