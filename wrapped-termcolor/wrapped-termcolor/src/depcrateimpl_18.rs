// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl fmt :: Display for ColorChoiceParseError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "unrecognized color choice '{}': valid choices are: \
             always, always-ansi, never, auto" , self . unknown_choice ,) } }
};
}
