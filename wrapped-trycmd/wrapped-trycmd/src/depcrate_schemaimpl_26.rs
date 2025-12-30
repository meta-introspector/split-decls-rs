// Generated macro for impl_26 (impl)
macro_rules! Depcrate_schemaimpl_26 {
() => {
// Module: crate::schema
// Provides: {"impl_26"}
// Dependencies: {}
impl JoinedArgs { # [cfg (test)] pub (crate) fn from_vec (inner : Vec < String >) -> Self { JoinedArgs { inner } } # [allow (clippy :: inherent_to_string_shadow_display)] fn to_string (& self) -> String { shlex :: join (self . inner . iter () . map (| s | s . as_str ())) } }
};
}
