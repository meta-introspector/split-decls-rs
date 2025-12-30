// Generated macro for impl_1145 (impl)
macro_rules! Depcrate_distimpl_1145 {
() => {
// Module: crate::dist
// Provides: {"impl_1145"}
// Dependencies: {}
impl FromStr for ServerId { type Err = < SocketAddr as FromStr > :: Err ; fn from_str (s : & str) -> :: std :: result :: Result < Self , Self :: Err > { SocketAddr :: from_str (s) . map (ServerId) } }
};
}
