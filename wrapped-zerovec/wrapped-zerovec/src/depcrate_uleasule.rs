// Generated macro for AsULE (trait)
macro_rules! Depcrate_uleAsULE {
() => {
// Module: crate::ule
// Provides: {"AsULE"}
// Dependencies: {}
# [doc = " A trait for any type that has a 1:1 mapping with an unaligned little-endian (ULE) type."] # [doc = ""] # [doc = " If you need to implement this trait, consider using [`#[make_ule]`](crate::make_ule) instead."] pub trait AsULE : Copy { # [doc = " The ULE type corresponding to `Self`."] # [doc = ""] # [doc = " Types having infallible conversions from all bit values (Plain Old Data) can use"] # [doc = " `RawBytesULE` with the desired width; for example, `u32` uses `RawBytesULE<4>`."] # [doc = ""] # [doc = " Types that are not well-defined for all bit values should implement a custom ULE."] type ULE : ULE ; # [doc = " Converts from `Self` to `Self::ULE`."] # [doc = ""] # [doc = " This function may involve byte order swapping (native-endian to little-endian)."] # [doc = ""] # [doc = " For best performance, mark your implementation of this function `#[inline]`."] fn to_unaligned (self) -> Self :: ULE ; # [doc = " Converts from `Self::ULE` to `Self`."] # [doc = ""] # [doc = " This function may involve byte order swapping (little-endian to native-endian)."] # [doc = ""] # [doc = " For best performance, mark your implementation of this function `#[inline]`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is infallible because bit validation should have occurred when `Self::ULE`"] # [doc = " was first constructed. An implementation may therefore involve an `unsafe{}` block, like"] # [doc = " `from_bytes_unchecked()`."] fn from_unaligned (unaligned : Self :: ULE) -> Self ; }
};
}
