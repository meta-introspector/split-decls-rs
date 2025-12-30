// Generated macro for hex (function)
macro_rules! Depcrate_msgs_basehex {
() => {
// Module: crate::msgs::base
// Provides: {"hex"}
// Dependencies: {}
pub (crate) fn hex < 'a > (f : & mut fmt :: Formatter < '_ > , payload : impl IntoIterator < Item = & 'a u8 > ,) -> fmt :: Result { for b in payload { write ! (f , "{b:02x}") ? ; } Ok (()) }
};
}
