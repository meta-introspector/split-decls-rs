// Generated macro for smallest_unsigned_type (function)
macro_rules! Depcrate_writersmallest_unsigned_type {
() => {
// Module: crate::writer
// Provides: {"smallest_unsigned_type"}
// Dependencies: {}
# [doc = " Return a string representing the smallest unsigned integer type for the"] # [doc = " given value."] fn smallest_unsigned_type (n : u64) -> & 'static str { if n <= :: std :: u8 :: MAX as u64 { "u8" } else if n <= :: std :: u16 :: MAX as u64 { "u16" } else if n <= :: std :: u32 :: MAX as u64 { "u32" } else { "u64" } }
};
}
