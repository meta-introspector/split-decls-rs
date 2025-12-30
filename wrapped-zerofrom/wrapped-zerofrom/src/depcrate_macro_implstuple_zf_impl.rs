// Generated macro for tuple_zf_impl (macro)
macro_rules! Depcrate_macro_implstuple_zf_impl {
() => {
// Module: crate::macro_impls
// Provides: {"tuple_zf_impl"}
// Dependencies: {}
macro_rules ! tuple_zf_impl { ($ (($ c : ident , $ t : ident , $ i : tt)) ,+) => { impl <'zf , $ ($ c , $ t : ZeroFrom <'zf , $ c >) ,+> ZeroFrom <'zf , ($ ($ c) ,+) > for ($ ($ t) ,+) { fn zero_from (other : &'zf ($ ($ c) ,+)) -> Self { ($ (<$ t as ZeroFrom <$ c >>:: zero_from (& other .$ i)) ,+) } } } ; }
};
}
