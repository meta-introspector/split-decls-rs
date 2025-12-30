// Generated macro for impl_details (module)
macro_rules! Depcrateimpl_details {
() => {
// Module: crate
// Provides: {"impl_details"}
// Dependencies: {}
# [cfg (feature = "gecko-ffi")] mod impl_details { pub type SizeType = u32 ; pub const MAX_CAP : usize = i32 :: max_value () as usize ; pub const AUTO_ARRAY_HEADER_OFFSET : usize = 8 ; # [cfg (target_endian = "little")] pub fn unpack_capacity (cap : SizeType) -> usize { (cap as usize) & ! (1 << 31) } # [cfg (target_endian = "little")] pub fn is_auto (cap : SizeType) -> bool { (cap & (1 << 31)) != 0 } # [cfg (target_endian = "little")] pub fn pack_capacity_and_auto (cap : SizeType , auto : bool) -> SizeType { cap | ((auto as SizeType) << 31) } # [cfg (target_endian = "big")] pub fn unpack_capacity (cap : SizeType) -> usize { (cap >> 1) as usize } # [cfg (target_endian = "big")] pub fn is_auto (cap : SizeType) -> bool { (cap & 1) != 0 } # [cfg (target_endian = "big")] pub fn pack_capacity_and_auto (cap : SizeType , auto : bool) -> SizeType { (cap << 1) | (auto as SizeType) } # [inline] pub fn assert_size (x : usize) -> SizeType { if x > MAX_CAP as usize { panic ! ("nsTArray size may not exceed the capacity of a 32-bit sized int") ; } x as SizeType } }
};
}
