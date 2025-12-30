// Generated macro for impl_688 (impl)
macro_rules! Depcrate_trustimpl_688 {
() => {
// Module: crate::trust
// Provides: {"impl_688"}
// Dependencies: {}
impl TrustResult { # [doc = " Returns true if the result is \"successful\" - specifically `PROCEED` or `UNSPECIFIED`."] # [inline] # [must_use] pub fn success (self) -> bool { matches ! (self , Self :: PROCEED | Self :: UNSPECIFIED) } }
};
}
