// Generated macro for impl_81 (impl)
macro_rules! Depcrate_fmtimpl_81 {
() => {
// Module: crate::fmt
// Provides: {"impl_81"}
// Dependencies: {}
impl FromStr for Braced { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { crate :: parser :: parse_braced (s . as_bytes ()) . map (| b | Braced (Uuid (b))) . map_err (| invalid | invalid . into_err ()) } }
};
}
