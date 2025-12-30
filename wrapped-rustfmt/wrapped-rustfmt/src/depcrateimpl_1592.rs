// Generated macro for impl_1592 (impl)
macro_rules! Depcrateimpl_1592 {
() => {
// Module: crate
// Provides: {"impl_1592"}
// Dependencies: {}
# [doc = " Deprecated - Use FormatReportFormatter instead"] impl fmt :: Display for FormatReport { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { write ! (fmt , "{}" , FormatReportFormatterBuilder :: new (self) . build ()) ? ; Ok (()) } }
};
}
