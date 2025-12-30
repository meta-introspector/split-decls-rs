// Generated macro for impl_160 (impl)
macro_rules! Depcrate_readimpl_160 {
() => {
// Module: crate::read
// Provides: {"impl_160"}
// Dependencies: {}
impl < 'a , R : Seek > SeekableTake < 'a , R > { pub fn new (inner : & 'a mut R , length : u64) -> io :: Result < Self > { let inner_starting_offset = inner . stream_position () ? ; Ok (Self { inner , inner_starting_offset , length , current_offset : 0 , }) } }
};
}
