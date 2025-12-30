// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < 'a , T > DerefMut for Locked < 'a , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . lock . value . get () } } }
};
}
