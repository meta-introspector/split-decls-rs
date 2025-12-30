// Generated macro for impl_474 (impl)
macro_rules! Depcrate_legacy_reduceimpl_474 {
() => {
// Module: crate::legacy::reduce
// Provides: {"impl_474"}
// Dependencies: {}
impl < R : Read > Read for ReduceDecoder < R > { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { if ! self . stream_read { self . stream_read = true ; let mut compressed_bytes = Vec :: new () ; self . compressed_reader . read_to_end (& mut compressed_bytes) ? ; hwexpand (& compressed_bytes , self . uncompressed_size as usize , self . comp_factor , & mut self . stream ,) ? ; } let available = self . stream . len () - self . read_pos ; let bytes_to_read = available . min (buf . len ()) ; buf [.. bytes_to_read] . copy_from_slice (& self . stream [self . read_pos .. self . read_pos + bytes_to_read]) ; self . read_pos += bytes_to_read ; Ok (bytes_to_read) } }
};
}
