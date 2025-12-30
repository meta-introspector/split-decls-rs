// Generated macro for impl_fmt_traits (macro)
macro_rules! Depcrate_byteorderimpl_fmt_traits {
() => {
// Module: crate::byteorder
// Provides: {"impl_fmt_traits"}
// Dependencies: {}
macro_rules ! impl_fmt_traits { ($ name : ident , $ native : ident , "floating point number") => { impl_fmt_trait ! ($ name , $ native , Display) ; } ; ($ name : ident , $ native : ident , "unsigned integer") => { impl_fmt_traits ! ($ name , $ native , @ all_types) ; } ; ($ name : ident , $ native : ident , "signed integer") => { impl_fmt_traits ! ($ name , $ native , @ all_types) ; } ; ($ name : ident , $ native : ident , @ all_types) => { impl_fmt_trait ! ($ name , $ native , Display) ; impl_fmt_trait ! ($ name , $ native , Octal) ; impl_fmt_trait ! ($ name , $ native , LowerHex) ; impl_fmt_trait ! ($ name , $ native , UpperHex) ; impl_fmt_trait ! ($ name , $ native , Binary) ; } ; }
};
}
