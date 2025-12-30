// Generated macro for impl_cmp (macro)
macro_rules! Depcrate_ffi_os_strimpl_cmp {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_cmp"}
// Dependencies: {}
macro_rules ! impl_cmp { ($ lhs : ty , $ rhs : ty) => { # [stable (feature = "cmp_os_str" , since = "1.8.0")] impl <'a , 'b > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { < OsStr as PartialEq >:: eq (self , other) } } # [stable (feature = "cmp_os_str" , since = "1.8.0")] impl <'a , 'b > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { < OsStr as PartialEq >:: eq (self , other) } } # [stable (feature = "cmp_os_str" , since = "1.8.0")] impl <'a , 'b > PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < cmp :: Ordering > { < OsStr as PartialOrd >:: partial_cmp (self , other) } } # [stable (feature = "cmp_os_str" , since = "1.8.0")] impl <'a , 'b > PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < cmp :: Ordering > { < OsStr as PartialOrd >:: partial_cmp (self , other) } } } ; }
};
}
