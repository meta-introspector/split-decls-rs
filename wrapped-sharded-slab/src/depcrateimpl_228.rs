// Generated macro for impl_228 (impl)
macro_rules! Depcrateimpl_228 {
() => {
// Module: crate
// Provides: {"impl_228"}
// Dependencies: {}
impl < T , C > OwnedEntry < T , C > where C : cfg :: Config , { # [doc = " Returns the key used to access this guard"] pub fn key (& self) -> usize { self . key } # [inline (always)] fn value (& self) -> & T { unsafe { self . value . as_ref () } } }
};
}
