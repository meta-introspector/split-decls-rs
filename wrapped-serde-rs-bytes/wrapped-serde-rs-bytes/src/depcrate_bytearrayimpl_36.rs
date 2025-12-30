// Generated macro for impl_36 (impl)
macro_rules! Depcrate_bytearrayimpl_36 {
() => {
// Module: crate::bytearray
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'de , const N : usize > Visitor < 'de > for BorrowedByteArrayVisitor < N > { type Value = & 'de ByteArray < N > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "a borrowed byte array of length {N}") } fn visit_borrowed_bytes < E > (self , v : & 'de [u8]) -> Result < Self :: Value , E > where E : Error , { let borrowed_byte_array : & 'de [u8 ; N] = v . try_into () . map_err (| _ | E :: invalid_length (v . len () , & self)) ? ; Ok (ByteArray :: from_ref (borrowed_byte_array)) } fn visit_borrowed_str < E > (self , v : & 'de str) -> Result < Self :: Value , E > where E : Error , { self . visit_borrowed_bytes (v . as_bytes ()) } }
};
}
