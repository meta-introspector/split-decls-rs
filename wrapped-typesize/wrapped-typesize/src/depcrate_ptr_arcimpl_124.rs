// Generated macro for impl_124 (impl)
macro_rules! Depcrate_ptr_arcimpl_124 {
() => {
// Module: crate::ptr::arc
// Provides: {"impl_124"}
// Dependencies: {}
impl < T , SC : ShouldCountInner > core :: ops :: Deref for SizableArc < T , SC > { type Target = Arc < T > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
};
}
