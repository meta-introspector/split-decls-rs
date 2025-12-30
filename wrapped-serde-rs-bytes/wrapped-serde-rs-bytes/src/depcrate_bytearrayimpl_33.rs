// Generated macro for impl_33 (impl)
macro_rules! Depcrate_bytearrayimpl_33 {
() => {
// Module: crate::bytearray
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'de , const N : usize > Visitor < 'de > for ByteArrayVisitor < N > { type Value = ByteArray < N > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "a byte array of length {N}") } fn visit_seq < V > (self , mut seq : V) -> Result < ByteArray < N > , V :: Error > where V : SeqAccess < 'de > , { let mut bytes = [0 ; N] ; for (idx , byte) in bytes . iter_mut () . enumerate () { * byte = seq . next_element () ? . ok_or_else (| | V :: Error :: invalid_length (idx , & self)) ? ; } Ok (ByteArray :: new (bytes)) } fn visit_bytes < E > (self , v : & [u8]) -> Result < ByteArray < N > , E > where E : Error , { Ok (ByteArray { bytes : v . try_into () . map_err (| _ | E :: invalid_length (v . len () , & self)) ? , }) } fn visit_str < E > (self , v : & str) -> Result < ByteArray < N > , E > where E : Error , { self . visit_bytes (v . as_bytes ()) } }
};
}
