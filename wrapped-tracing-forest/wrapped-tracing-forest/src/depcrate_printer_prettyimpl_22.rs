// Generated macro for impl_22 (impl)
macro_rules! Depcrate_printer_prettyimpl_22 {
() => {
// Module: crate::printer::pretty
// Provides: {"impl_22"}
// Dependencies: {}
impl fmt :: Display for DurationDisplay { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut t = self . 0 ; for unit in ["ns" , "µs" , "ms" , "s"] { if t < 10.0 { return write ! (f , "{t:.2}{unit}") ; } else if t < 100.0 { return write ! (f , "{t:.1}{unit}") ; } else if t < 1000.0 { return write ! (f , "{t:.0}{unit}") ; } t /= 1000.0 ; } write ! (f , "{:.0}s" , t * 1000.0) } }
};
}
