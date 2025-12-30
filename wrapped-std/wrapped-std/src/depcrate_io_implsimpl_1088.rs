// Generated macro for impl_1088 (impl)
macro_rules! Depcrate_io_implsimpl_1088 {
() => {
// Module: crate::io::impls
// Provides: {"impl_1088"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < B : BufRead + ? Sized > BufRead for Box < B > { # [inline] fn fill_buf (& mut self) -> io :: Result < & [u8] > { (* * self) . fill_buf () } # [inline] fn consume (& mut self , amt : usize) { (* * self) . consume (amt) } # [inline] fn has_data_left (& mut self) -> io :: Result < bool > { (* * self) . has_data_left () } # [inline] fn read_until (& mut self , byte : u8 , buf : & mut Vec < u8 >) -> io :: Result < usize > { (* * self) . read_until (byte , buf) } # [inline] fn skip_until (& mut self , byte : u8) -> io :: Result < usize > { (* * self) . skip_until (byte) } # [inline] fn read_line (& mut self , buf : & mut String) -> io :: Result < usize > { (* * self) . read_line (buf) } }
};
}
