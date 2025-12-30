// Generated macro for ZeroedWithSize (trait)
macro_rules! Depcrate_verification_windowsZeroedWithSize {
() => {
// Module: crate::verification::windows
// Provides: {"ZeroedWithSize"}
// Dependencies: {}
# [doc = " A trait to represent an object that can be safely created with all zero values"] # [doc = " and have a size assigned to it."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This has the same safety requirements as [std::mem::zeroed]."] unsafe trait ZeroedWithSize : Sized { const SIZE : u32 = { let size = core :: mem :: size_of :: < Self > () ; # [allow (clippy :: as_conversions)] if size <= u32 :: MAX as usize { size as u32 } else { panic ! ("structure was larger then DWORD") } } ; # [doc = " Returns a zeroed structure with its structure size (`cbSize`) field set to the correct value."] fn zeroed_with_size () -> Self ; }
};
}
