// Generated macro for impl_113 (impl)
macro_rules! Depcrate_hyperlinkimpl_113 {
() => {
// Module: crate::hyperlink
// Provides: {"impl_113"}
// Dependencies: {}
impl < T > PaintedLink < T > { fn fmt_args (& self , fmt : & dyn Fn (& Painted < T > , & mut fmt :: Formatter) -> fmt :: Result , f : & mut fmt :: Formatter , _args : fmt :: Arguments < '_ > ,) -> fmt :: Result { if ! self . painted . enabled () { return fmt (& self . painted , f) ; } write ! (f , "\x1B]8;;{}\x1B\\" , self . link) ? ; fmt (& self . painted , f) ? ; write ! (f , "\x1B]8;;\x1B\\") } }
};
}
