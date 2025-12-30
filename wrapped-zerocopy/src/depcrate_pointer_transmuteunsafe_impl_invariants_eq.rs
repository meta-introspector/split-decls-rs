// Generated macro for unsafe_impl_invariants_eq (macro)
macro_rules! Depcrate_pointer_transmuteunsafe_impl_invariants_eq {
() => {
// Module: crate::pointer::transmute
// Provides: {"unsafe_impl_invariants_eq"}
// Dependencies: {}
macro_rules ! unsafe_impl_invariants_eq { ($ tyvar : ident => $ t : ty , $ u : ty) => { { crate :: util :: macros :: __unsafe () ; unsafe impl <$ tyvar > InvariantsEq <$ t > for $ u { } unsafe impl <$ tyvar > InvariantsEq <$ u > for $ t { } } } ; }
};
}
