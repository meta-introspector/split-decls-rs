// Generated macro for impl_135 (impl)
macro_rules! Depcrate_ptr_rcimpl_135 {
() => {
// Module: crate::ptr::rc
// Provides: {"impl_135"}
// Dependencies: {}
impl < T , SC : ShouldCountInner > core :: ops :: Deref for SizableRc < T , SC > { type Target = Rc < T > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
};
}
