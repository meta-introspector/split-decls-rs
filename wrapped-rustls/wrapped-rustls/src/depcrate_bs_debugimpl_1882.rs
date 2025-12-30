// Generated macro for impl_1882 (impl)
macro_rules! Depcrate_bs_debugimpl_1882 {
() => {
// Module: crate::bs_debug
// Provides: {"impl_1882"}
// Dependencies: {}
impl fmt :: Debug for BsDebug < '_ > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { write ! (fmt , "b\"") ? ; for & c in self . 0 { if c == b'\n' { write ! (fmt , "\\n") ? ; } else if c == b'\r' { write ! (fmt , "\\r") ? ; } else if c == b'\t' { write ! (fmt , "\\t") ? ; } else if c == b'\\' || c == b'"' { write ! (fmt , "\\{}" , c as char) ? ; } else if c == b'\0' { write ! (fmt , "\\0") ? ; } else if (0x20 .. 0x7f) . contains (& c) { write ! (fmt , "{}" , c as char) ? ; } else { write ! (fmt , "\\x{c:02x}") ? ; } } write ! (fmt , "\"") ? ; Ok (()) } }
};
}
