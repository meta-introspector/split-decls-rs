// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a , T > Deref for Locked < 'a , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . lock . value . get () } } }
};
}
