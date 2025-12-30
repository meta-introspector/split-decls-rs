// Generated macro for impl_100 (impl)
macro_rules! Depcrate_linked_listimpl_100 {
() => {
// Module: crate::linked_list
// Provides: {"impl_100"}
// Dependencies: {}
impl < T > Deref for LinkedEntry < T > { type Target = T ; # [inline] fn deref (& self) -> & Self :: Target { unsafe { self . instance . as_ref () . unwrap_unchecked () } } }
};
}
