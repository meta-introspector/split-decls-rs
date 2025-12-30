// Generated macro for impl_91 (impl)
macro_rules! Depcrateimpl_91 {
() => {
// Module: crate
// Provides: {"impl_91"}
// Dependencies: {}
impl Reset for Sha1 { # [inline] fn reset (& mut self) { self . h = INITIAL_H ; self . block_len = 0 ; self . buffer . reset () ; if let Some (ref mut ctx) = self . detection { ctx . reset () ; } } }
};
}
