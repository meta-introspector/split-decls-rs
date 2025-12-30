// Generated macro for impl_80 (impl)
macro_rules! Depcrate_fmtimpl_80 {
() => {
// Module: crate::fmt
// Provides: {"impl_80"}
// Dependencies: {}
impl FromStr for Urn { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { crate :: parser :: parse_urn (s . as_bytes ()) . map (| b | Urn (Uuid (b))) . map_err (| invalid | invalid . into_err ()) } }
};
}
