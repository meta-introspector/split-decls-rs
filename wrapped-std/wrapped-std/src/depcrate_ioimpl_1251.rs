// Generated macro for impl_1251 (impl)
macro_rules! Depcrate_ioimpl_1251 {
() => {
// Module: crate::io
// Provides: {"impl_1251"}
// Dependencies: {}
# [stable (feature = "chain_bufread" , since = "1.9.0")] impl < T : BufRead , U : BufRead > BufRead for Chain < T , U > { fn fill_buf (& mut self) -> Result < & [u8] > { if ! self . done_first { match self . first . fill_buf () ? { buf if buf . is_empty () => self . done_first = true , buf => return Ok (buf) , } } self . second . fill_buf () } fn consume (& mut self , amt : usize) { if ! self . done_first { self . first . consume (amt) } else { self . second . consume (amt) } } fn read_until (& mut self , byte : u8 , buf : & mut Vec < u8 >) -> Result < usize > { let mut read = 0 ; if ! self . done_first { let n = self . first . read_until (byte , buf) ? ; read += n ; match buf . last () { Some (b) if * b == byte && n != 0 => return Ok (read) , _ => self . done_first = true , } } read += self . second . read_until (byte , buf) ? ; Ok (read) } }
};
}
