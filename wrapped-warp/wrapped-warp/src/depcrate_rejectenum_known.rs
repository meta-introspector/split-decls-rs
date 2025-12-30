// Generated macro for enum_known (macro)
macro_rules! Depcrate_rejectenum_known {
() => {
// Module: crate::reject
// Provides: {"enum_known"}
// Dependencies: {}
macro_rules ! enum_known { ($ ($ (# [$ attr : meta]) * $ var : ident ($ ty : path) ,) +) => (pub (crate) enum Known { $ ($ (# [$ attr]) * $ var ($ ty) ,) + } impl Known { fn inner_as_any (& self) -> & dyn Any { match * self { $ ($ (# [$ attr]) * Known ::$ var (ref t) => t ,) + } } } impl fmt :: Debug for Known { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { match * self { $ ($ (# [$ attr]) * Known ::$ var (ref t) => t . fmt (f) ,) + } } } impl fmt :: Display for Known { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { match * self { $ ($ (# [$ attr]) * Known ::$ var (ref t) => t . fmt (f) ,) + } } } $ (# [doc (hidden)] $ (# [$ attr]) * impl From <$ ty > for Known { fn from (ty : $ ty) -> Known { Known ::$ var (ty) } }) +) ; }
};
}
