// Generated macro for impl_96 (impl)
macro_rules! Depcrateimpl_96 {
() => {
// Module: crate
// Provides: {"impl_96"}
// Dependencies: {}
impl FixedOutputReset for Sha1 { # [inline] fn finalize_into_reset (& mut self , out : & mut Output < Self >) { self . finalize_inner (out) ; Reset :: reset (self) ; } }
};
}
