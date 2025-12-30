// Generated macro for impl_1250 (impl)
macro_rules! Depcrate_ioimpl_1250 {
() => {
// Module: crate::io
// Provides: {"impl_1250"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Read , U : Read > Read for Chain < T , U > { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { if ! self . done_first { match self . first . read (buf) ? { 0 if ! buf . is_empty () => self . done_first = true , n => return Ok (n) , } } self . second . read (buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> Result < usize > { if ! self . done_first { match self . first . read_vectored (bufs) ? { 0 if bufs . iter () . any (| b | ! b . is_empty ()) => self . done_first = true , n => return Ok (n) , } } self . second . read_vectored (bufs) } # [inline] fn is_read_vectored (& self) -> bool { self . first . is_read_vectored () || self . second . is_read_vectored () } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> Result < usize > { let mut read = 0 ; if ! self . done_first { read += self . first . read_to_end (buf) ? ; self . done_first = true ; } read += self . second . read_to_end (buf) ? ; Ok (read) } fn read_buf (& mut self , mut buf : BorrowedCursor < '_ >) -> Result < () > { if buf . capacity () == 0 { return Ok (()) ; } if ! self . done_first { let old_len = buf . written () ; self . first . read_buf (buf . reborrow ()) ? ; if buf . written () != old_len { return Ok (()) ; } else { self . done_first = true ; } } self . second . read_buf (buf) } }
};
}
