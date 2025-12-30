// Generated macro for impl_1034 (impl)
macro_rules! Depcrate_io_cursorimpl_1034 {
() => {
// Module: crate::io::cursor
// Provides: {"impl_1034"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > BufRead for Cursor < T > where T : AsRef < [u8] > , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { Ok (Cursor :: split (self) . 1) } fn consume (& mut self , amt : usize) { self . pos += amt as u64 ; } }
};
}
