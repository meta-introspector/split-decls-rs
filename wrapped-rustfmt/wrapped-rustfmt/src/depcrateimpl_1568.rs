// Generated macro for impl_1568 (impl)
macro_rules! Depcrateimpl_1568 {
() => {
// Module: crate
// Provides: {"impl_1568"}
// Dependencies: {}
# [doc = " Deprecated - Use FormatReportFormatter instead"] impl fmt :: Display for FormatReport { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { write ! (fmt , "{}" , FormatReportFormatterBuilder :: new (self) . build ()) ? ; Ok (()) } }
};
}
