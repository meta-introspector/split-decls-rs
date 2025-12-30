// Generated macro for impl_898 (impl)
macro_rules! Depcrate_io_buffered_bufreaderimpl_898 {
() => {
// Module: crate::io::buffered::bufreader
// Provides: {"impl_898"}
// Dependencies: {}
impl < R > SpecReadByte for BufReader < R > where Self : Read , { # [inline] fn spec_read_byte (& mut self) -> Option < io :: Result < u8 > > { let mut byte = 0 ; if self . buf . consume_with (1 , | claimed | byte = claimed [0]) { return Some (Ok (byte)) ; } uninlined_slow_read_byte (self) } }
};
}
