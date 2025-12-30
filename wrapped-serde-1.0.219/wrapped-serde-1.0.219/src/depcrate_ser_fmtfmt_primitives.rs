// Generated macro for fmt_primitives (macro)
macro_rules! Depcrate_ser_fmtfmt_primitives {
() => {
// Module: crate::ser::fmt
// Provides: {"fmt_primitives"}
// Dependencies: {}
macro_rules ! fmt_primitives { ($ ($ f : ident : $ t : ty ,) *) => { $ (fn $ f (self , v : $ t) -> fmt :: Result { Display :: fmt (& v , self) }) * } ; }
};
}
