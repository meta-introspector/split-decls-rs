// Generated macro for impl_24 (impl)
macro_rules! Depcrate_printer_prettyimpl_24 {
() => {
// Module: crate::printer::pretty
// Provides: {"impl_24"}
// Dependencies: {}
impl fmt :: Display for FieldKey < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (feature = "ansi")] { let color = Color :: White . dimmed () ; write ! (f , "{}{}{}" , color . prefix () , self . 0 , color . suffix ()) } # [cfg (not (feature = "ansi"))] { f . write_str (self . 0) } } }
};
}
