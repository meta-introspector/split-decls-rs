// Generated macro for impl_12 (impl)
macro_rules! Depcrate_arraysimpl_12 {
() => {
// Module: crate::arrays
// Provides: {"impl_12"}
// Dependencies: {}
impl < const LEN : usize > DeserializeBytes for [u8 ; LEN] { # [inline] fn tls_deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > { let out = bytes . get (.. LEN) . ok_or (Error :: EndOfStream) ? . try_into () . map_err (| _ | Error :: EndOfStream) ? ; Ok ((out , & bytes [LEN ..])) } }
};
}
