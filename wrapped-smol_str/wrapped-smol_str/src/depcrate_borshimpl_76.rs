// Generated macro for impl_76 (impl)
macro_rules! Depcrate_borshimpl_76 {
() => {
// Module: crate::borsh
// Provides: {"impl_76"}
// Dependencies: {}
impl BorshDeserialize for SmolStr { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> borsh :: io :: Result < Self > { let len = u32 :: deserialize_reader (reader) ? ; if (len as usize) < INLINE_CAP { let mut buf = [0u8 ; INLINE_CAP] ; reader . read_exact (& mut buf [.. len as usize]) ? ; _ = core :: str :: from_utf8 (& buf [.. len as usize]) . map_err (| err | { let msg = err . to_string () ; Error :: new (ErrorKind :: InvalidData , msg) }) ? ; Ok (SmolStr (Repr :: Inline { len : unsafe { transmute :: < u8 , crate :: InlineSize > (len as u8) } , buf , })) } else { let vec = u8 :: vec_from_reader (len , reader) ? . ok_or_else (| | Error :: other ("u8::vec_from_reader unexpectedly returned None")) ? ; Ok (SmolStr :: from (String :: from_utf8 (vec) . map_err (| err | { let msg = err . to_string () ; Error :: new (ErrorKind :: InvalidData , msg) }) ?)) } } }
};
}
