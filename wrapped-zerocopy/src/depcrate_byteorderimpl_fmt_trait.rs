// Generated macro for impl_fmt_trait (macro)
macro_rules! Depcrate_byteorderimpl_fmt_trait {
() => {
// Module: crate::byteorder
// Provides: {"impl_fmt_trait"}
// Dependencies: {}
macro_rules ! impl_fmt_trait { ($ name : ident , $ native : ident , $ trait : ident) => { impl < O : ByteOrder > $ trait for $ name < O > { # [inline (always)] fn fmt (& self , f : & mut Formatter <'_ >) -> fmt :: Result { $ trait :: fmt (& self . get () , f) } } } ; }
};
}
