// Generated macro for ToUsize (trait)
macro_rules! Depcrate_streamToUsize {
() => {
// Module: crate::stream
// Provides: {"ToUsize"}
// Dependencies: {}
# [doc = " Helper trait to convert numbers to usize."] # [doc = ""] # [doc = " By default, usize implements `From<u8>` and `From<u16>` but not"] # [doc = " `From<u32>` and `From<u64>` because that would be invalid on some"] # [doc = " platforms. This trait implements the conversion for platforms"] # [doc = " with 32 and 64 bits pointer platforms"] pub trait ToUsize { # [doc = " converts self to usize"] fn to_usize (& self) -> usize ; }
};
}
