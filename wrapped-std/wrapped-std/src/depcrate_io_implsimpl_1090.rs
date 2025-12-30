// Generated macro for impl_1090 (impl)
macro_rules! Depcrate_io_implsimpl_1090 {
() => {
// Module: crate::io::impls
// Provides: {"impl_1090"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl BufRead for & [u8] { # [inline] fn fill_buf (& mut self) -> io :: Result < & [u8] > { Ok (* self) } # [inline] fn consume (& mut self , amt : usize) { * self = & self [amt ..] ; } }
};
}
