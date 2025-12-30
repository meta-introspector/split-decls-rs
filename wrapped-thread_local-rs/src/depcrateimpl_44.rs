// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
impl < T > Drop for Entry < T > { fn drop (& mut self) { if * self . present . get_mut () { unsafe { MaybeUninit :: assume_init_drop (& mut * self . value . get ()) ; } } } }
};
}
