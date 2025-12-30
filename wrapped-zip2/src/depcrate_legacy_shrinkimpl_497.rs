// Generated macro for impl_497 (impl)
macro_rules! Depcrate_legacy_shrinkimpl_497 {
() => {
// Module: crate::legacy::shrink
// Provides: {"impl_497"}
// Dependencies: {}
impl < R : Read > Read for ShrinkDecoder < R > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { if ! self . stream_read { self . stream_read = true ; let mut compressed_bytes = Vec :: new () ; self . compressed_reader . read_to_end (& mut compressed_bytes) ? ; self . stream . reserve (self . uncompressed_size as usize) ; hwunshrink (& compressed_bytes , self . uncompressed_size as usize , & mut self . stream ,) ? ; } let available = self . stream . len () . saturating_sub (self . read_pos) ; if available == 0 { return Ok (0) ; } let n = available . min (buf . len ()) ; buf [.. n] . copy_from_slice (& self . stream [self . read_pos .. self . read_pos + n]) ; self . read_pos += n ; Ok (n) } }
};
}
