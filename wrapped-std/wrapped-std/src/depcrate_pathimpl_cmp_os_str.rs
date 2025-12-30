// Generated macro for impl_cmp_os_str (macro)
macro_rules! Depcrate_pathimpl_cmp_os_str {
() => {
// Module: crate::path
// Provides: {"impl_cmp_os_str"}
// Dependencies: {}
macro_rules ! impl_cmp_os_str { (<$ ($ life : lifetime) ,*> $ lhs : ty , $ rhs : ty) => { # [stable (feature = "cmp_path" , since = "1.8.0")] impl <$ ($ life) ,*> PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { < Path as PartialEq >:: eq (self , other . as_ref ()) } } # [stable (feature = "cmp_path" , since = "1.8.0")] impl <$ ($ life) ,*> PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { < Path as PartialEq >:: eq (self . as_ref () , other) } } # [stable (feature = "cmp_path" , since = "1.8.0")] impl <$ ($ life) ,*> PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < cmp :: Ordering > { < Path as PartialOrd >:: partial_cmp (self , other . as_ref ()) } } # [stable (feature = "cmp_path" , since = "1.8.0")] impl <$ ($ life) ,*> PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < cmp :: Ordering > { < Path as PartialOrd >:: partial_cmp (self . as_ref () , other) } } } ; }
};
}
