// Generated macro for impl_1151 (impl)
macro_rules! Depcrate_io_stdioimpl_1151 {
() => {
// Module: crate::io::stdio
// Provides: {"impl_1151"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl BufRead for StdinLock < '_ > { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . inner . fill_buf () } fn consume (& mut self , n : usize) { self . inner . consume (n) } fn read_until (& mut self , byte : u8 , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . inner . read_until (byte , buf) } fn read_line (& mut self , buf : & mut String) -> io :: Result < usize > { self . inner . read_line (buf) } }
};
}
