// Generated macro for impl_643 (impl)
macro_rules! Depcrate_lookaheadimpl_643 {
() => {
// Module: crate::lookahead
// Provides: {"impl_643"}
// Dependencies: {}
impl < 'a > Display for CommaSeparated < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut first = true ; for & s in self . 0 { if ! first { f . write_str (", ") ? ; } f . write_str (s) ? ; first = false ; } Ok (()) } }
};
}
