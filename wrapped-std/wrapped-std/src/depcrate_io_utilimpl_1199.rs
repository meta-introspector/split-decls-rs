// Generated macro for impl_1199 (impl)
macro_rules! Depcrate_io_utilimpl_1199 {
() => {
// Module: crate::io::util
// Provides: {"impl_1199"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl BufRead for Empty { # [inline] fn fill_buf (& mut self) -> io :: Result < & [u8] > { Ok (& []) } # [inline] fn consume (& mut self , _n : usize) { } # [inline] fn has_data_left (& mut self) -> io :: Result < bool > { Ok (false) } # [inline] fn read_until (& mut self , _byte : u8 , _buf : & mut Vec < u8 >) -> io :: Result < usize > { Ok (0) } # [inline] fn skip_until (& mut self , _byte : u8) -> io :: Result < usize > { Ok (0) } # [inline] fn read_line (& mut self , _buf : & mut String) -> io :: Result < usize > { Ok (0) } }
};
}
