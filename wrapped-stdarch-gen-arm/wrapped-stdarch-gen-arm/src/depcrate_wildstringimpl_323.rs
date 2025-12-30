// Generated macro for impl_323 (impl)
macro_rules! Depcrate_wildstringimpl_323 {
() => {
// Module: crate::wildstring
// Provides: {"impl_323"}
// Dependencies: {}
impl fmt :: Display for WildString { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . 0 . iter () . map (| part | match part { WildStringPart :: String (s) => s . to_owned () , WildStringPart :: Wildcard (w) => format ! ("{{{w}}}") , }) . join ("")) } }
};
}
