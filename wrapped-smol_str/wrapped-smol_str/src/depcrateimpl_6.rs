// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl Clone for SmolStr { # [inline] fn clone (& self) -> Self { # [cold] # [inline (never)] fn cold_clone (v : & SmolStr) -> SmolStr { SmolStr (v . 0 . clone ()) } if self . is_heap_allocated () { return cold_clone (self) ; } unsafe { core :: ptr :: read (self as * const SmolStr) } } }
};
}
