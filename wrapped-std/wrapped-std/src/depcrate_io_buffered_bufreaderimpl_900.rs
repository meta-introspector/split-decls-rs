// Generated macro for impl_900 (impl)
macro_rules! Depcrate_io_buffered_bufreaderimpl_900 {
() => {
// Module: crate::io::buffered::bufreader
// Provides: {"impl_900"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < R : ? Sized + Read > BufRead for BufReader < R > { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . buf . fill_buf (& mut self . inner) } fn consume (& mut self , amt : usize) { self . buf . consume (amt) } }
};
}
