// Generated macro for impl_79 (impl)
macro_rules! Depcrate_fmtimpl_79 {
() => {
// Module: crate::fmt
// Provides: {"impl_79"}
// Dependencies: {}
impl FromStr for Simple { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { crate :: parser :: parse_simple (s . as_bytes ()) . map (| b | Simple (Uuid (b))) . map_err (| invalid | invalid . into_err ()) } }
};
}
