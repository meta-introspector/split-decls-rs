// Generated macro for impl_25 (impl)
macro_rules! Depcrate_errorimpl_25 {
() => {
// Module: crate::error
// Provides: {"impl_25"}
// Dependencies: {}
impl Display for QuotedChar { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { if self . 0 == '\0' { formatter . write_str ("'\\0'") } else { write ! (formatter , "{:?}" , self . 0) } } }
};
}
