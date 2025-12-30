// Generated macro for impl_1150 (impl)
macro_rules! Depcrate_io_stdioimpl_1150 {
() => {
// Module: crate::io::stdio
// Provides: {"impl_1150"}
// Dependencies: {}
impl SpecReadByte for StdinLock < '_ > { # [inline] fn spec_read_byte (& mut self) -> Option < io :: Result < u8 > > { BufReader :: spec_read_byte (& mut * self . inner) } }
};
}
