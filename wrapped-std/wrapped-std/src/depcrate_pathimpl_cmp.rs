// Generated macro for impl_cmp (macro)
macro_rules! Depcrate_pathimpl_cmp {
() => {
// Module: crate::path
// Provides: {"impl_cmp"}
// Dependencies: {}
macro_rules ! impl_cmp { (<$ ($ life : lifetime) ,*> $ lhs : ty , $ rhs : ty) => { # [stable (feature = "partialeq_path" , since = "1.6.0")] impl <$ ($ life) ,*> PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { < Path as PartialEq >:: eq (self , other) } } # [stable (feature = "partialeq_path" , since = "1.6.0")] impl <$ ($ life) ,*> PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { < Path as PartialEq >:: eq (self , other) } } # [stable (feature = "cmp_path" , since = "1.8.0")] impl <$ ($ life) ,*> PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < cmp :: Ordering > { < Path as PartialOrd >:: partial_cmp (self , other) } } # [stable (feature = "cmp_path" , since = "1.8.0")] impl <$ ($ life) ,*> PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < cmp :: Ordering > { < Path as PartialOrd >:: partial_cmp (self , other) } } } ; }
};
}
