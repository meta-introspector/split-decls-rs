// Generated macro for write_hex (function)
macro_rules! Depcrate_quic_vecwrite_hex {
() => {
// Module: crate::quic_vec
// Provides: {"write_hex"}
// Dependencies: {}
fn write_hex (f : & mut fmt :: Formatter < '_ > , data : & [u8]) -> fmt :: Result { if ! data . is_empty () { write ! (f , "0x") ? ; for byte in data { write ! (f , "{byte:02x}") ? ; } } else { write ! (f , "b\"\"") ? ; } Ok (()) }
};
}
