// Generated macro for impl_153 (impl)
macro_rules! Depcrate_fieldimpl_153 {
() => {
// Module: crate::field
// Provides: {"impl_153"}
// Dependencies: {}
impl fmt :: Debug for HexBytes < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_char ('[') ? ; let mut bytes = self . 0 . iter () ; if let Some (byte) = bytes . next () { f . write_fmt (format_args ! ("{byte:02x}")) ? ; } for byte in bytes { f . write_fmt (format_args ! (" {byte:02x}")) ? ; } f . write_char (']') } }
};
}
