// Generated macro for impl_partial_ord (macro)
macro_rules! Depcrate_macrosimpl_partial_ord {
() => {
// Module: crate::macros
// Provides: {"impl_partial_ord"}
// Dependencies: {}
macro_rules ! impl_partial_ord { ($ lhs : ty , $ rhs : ty) => { # [allow (unused_lifetimes)] impl <'a > PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < Ordering > { let l = self ; let r : & Self = other . as_ref () ; PartialOrd :: partial_cmp (l , r) } } # [allow (unused_lifetimes)] impl <'a > PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < Ordering > { PartialOrd :: partial_cmp (other , self) } } } ; }
};
}
