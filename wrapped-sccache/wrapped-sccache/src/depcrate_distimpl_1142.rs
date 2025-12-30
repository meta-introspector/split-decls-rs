// Generated macro for impl_1142 (impl)
macro_rules! Depcrate_distimpl_1142 {
() => {
// Module: crate::dist
// Provides: {"impl_1142"}
// Dependencies: {}
impl FromStr for JobId { type Err = < u64 as FromStr > :: Err ; fn from_str (s : & str) -> :: std :: result :: Result < Self , Self :: Err > { u64 :: from_str (s) . map (JobId) } }
};
}
