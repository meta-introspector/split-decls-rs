// Generated macro for impl_78 (impl)
macro_rules! Depcrate_fmtimpl_78 {
() => {
// Module: crate::fmt
// Provides: {"impl_78"}
// Dependencies: {}
impl FromStr for Hyphenated { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { crate :: parser :: parse_hyphenated (s . as_bytes ()) . map (| b | Hyphenated (Uuid (b))) . map_err (| invalid | invalid . into_err ()) } }
};
}
