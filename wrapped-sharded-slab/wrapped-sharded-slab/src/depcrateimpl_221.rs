// Generated macro for impl_221 (impl)
macro_rules! Depcrateimpl_221 {
() => {
// Module: crate
// Provides: {"impl_221"}
// Dependencies: {}
impl < T , C : cfg :: Config > Entry < '_ , T , C > { # [doc = " Returns the key used to access the guard."] pub fn key (& self) -> usize { self . key } # [inline (always)] fn value (& self) -> & T { unsafe { self . value . as_ref () } } }
};
}
