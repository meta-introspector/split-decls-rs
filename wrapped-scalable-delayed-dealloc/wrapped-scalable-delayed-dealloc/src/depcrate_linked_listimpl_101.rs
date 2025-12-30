// Generated macro for impl_101 (impl)
macro_rules! Depcrate_linked_listimpl_101 {
() => {
// Module: crate::linked_list
// Provides: {"impl_101"}
// Dependencies: {}
impl < T > DerefMut for LinkedEntry < T > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { self . instance . as_mut () . unwrap_unchecked () } } }
};
}
