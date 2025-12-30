// Generated macro for impl_172 (impl)
macro_rules! Depcrate_serial_numberimpl_172 {
() => {
// Module: crate::serial_number
// Provides: {"impl_172"}
// Dependencies: {}
impl Display for SerialNumber { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let mut iter = self . as_bytes () . iter () . peekable () ; while let Some (byte) = iter . next () { match iter . peek () { Some (_) => write ! (f , "{byte:02X}:") ? , None => write ! (f , "{byte:02X}") ? , } } Ok (()) } }
};
}
