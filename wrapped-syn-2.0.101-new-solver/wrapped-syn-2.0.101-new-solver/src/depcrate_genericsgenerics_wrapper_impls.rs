// Generated macro for generics_wrapper_impls (macro)
macro_rules! Depcrate_genericsgenerics_wrapper_impls {
() => {
// Module: crate::generics
// Provides: {"generics_wrapper_impls"}
// Dependencies: {}
# [cfg (feature = "printing")] macro_rules ! generics_wrapper_impls { ($ ty : ident) => { # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl <'a > Clone for $ ty <'a > { fn clone (& self) -> Self { $ ty (self . 0) } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl <'a > Debug for $ ty <'a > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_tuple (stringify ! ($ ty)) . field (self . 0) . finish () } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl <'a > Eq for $ ty <'a > { } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl <'a > PartialEq for $ ty <'a > { fn eq (& self , other : & Self) -> bool { self . 0 == other . 0 } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl <'a > Hash for $ ty <'a > { fn hash < H : Hasher > (& self , state : & mut H) { self . 0 . hash (state) ; } } } ; }
};
}
