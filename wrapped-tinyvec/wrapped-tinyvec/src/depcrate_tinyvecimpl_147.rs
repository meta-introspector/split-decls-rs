// Generated macro for impl_147 (impl)
macro_rules! Depcrate_tinyvecimpl_147 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_147"}
// Dependencies: {}
impl < A : Array > Deref for TinyVec < A > { type Target = [A :: Item] ; impl_mirrored ! { type Mirror = TinyVec ; # [inline (always)] # [must_use] fn deref (self : & Self) -> & Self :: Target ; } }
};
}
