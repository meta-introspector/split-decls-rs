// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl From < Arc < str > > for SmolStr { # [inline] fn from (s : Arc < str >) -> SmolStr { let repr = Repr :: new_on_stack (s . as_ref ()) . unwrap_or (Repr :: Heap (s)) ; Self (repr) } }
};
}
