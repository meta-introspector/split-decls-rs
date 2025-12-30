// Generated macro for impl_partial_eq (macro)
macro_rules! Depcrate_macrosimpl_partial_eq {
() => {
// Module: crate::macros
// Provides: {"impl_partial_eq"}
// Dependencies: {}
macro_rules ! impl_partial_eq { ($ lhs : ty , $ rhs : ty) => { # [allow (unused_lifetimes)] impl <'a > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { let l = self ; let r : & Self = other . as_ref () ; PartialEq :: eq (l , r) } } # [allow (unused_lifetimes)] impl <'a > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { PartialEq :: eq (other , self) } } } ; }
};
}
