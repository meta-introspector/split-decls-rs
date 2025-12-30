// Generated macro for memchr3 (function)
macro_rules! Depcrate_streammemchr3 {
() => {
// Module: crate::stream
// Provides: {"memchr3"}
// Dependencies: {}
# [cfg (not (feature = "simd"))] # [inline (always)] fn memchr3 (token : (u8 , u8 , u8) , slice : & [u8]) -> Option < usize > { slice . iter () . position (| t | * t == token . 0 || * t == token . 1 || * t == token . 2) }
};
}
