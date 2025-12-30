// Generated macro for impl_26 (impl)
macro_rules! Depcrate_printer_prettyimpl_26 {
() => {
// Module: crate::printer::pretty
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (feature = "ansi")] impl fmt :: Display for ColorLevel { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let color = match self . 0 { Level :: TRACE => Color :: Purple , Level :: DEBUG => Color :: Blue , Level :: INFO => Color :: Green , Level :: WARN => Color :: RGB (252 , 234 , 160) , Level :: ERROR => Color :: Red , } ; let style = color . bold () ; write ! (f , "{}" , style . prefix ()) ? ; f . pad (self . 0 . as_str ()) ? ; write ! (f , "{}" , style . suffix ()) } }
};
}
