// Generated macro for impl_123 (impl)
macro_rules! Depcrate_headerimpl_123 {
() => {
// Module: crate::header
// Provides: {"impl_123"}
// Dependencies: {}
impl < H > Arc < HeaderSlice < H , str > > { # [doc = " Creates an Arc for a HeaderSlice using the given header struct and"] # [doc = " a str slice to generate the slice. The resulting Arc will be fat."] pub fn from_header_and_str (header : H , string : & str) -> Self { let bytes = Arc :: from_header_and_slice (header , string . as_bytes ()) ; unsafe { Arc :: from_raw_inner (Arc :: into_raw_inner (bytes) as _) } } }
};
}
