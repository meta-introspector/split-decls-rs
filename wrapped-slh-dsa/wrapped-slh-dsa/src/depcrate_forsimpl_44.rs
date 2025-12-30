// Generated macro for impl_44 (impl)
macro_rules! Depcrate_forsimpl_44 {
() => {
// Module: crate::fors
// Provides: {"impl_44"}
// Dependencies: {}
impl < P : ForsParams > TryFrom < & [u8] > for ForsSignature < P > { type Error = () ; fn try_from (slice : & [u8]) -> Result < Self , Self :: Error > { if slice . len () != Self :: SIZE { return Err (()) ; } Ok (Self (slice . chunks (ForsMTSig :: < P > :: SIZE) . map (| c | c . try_into () . unwrap ()) . collect () ,)) } }
};
}
