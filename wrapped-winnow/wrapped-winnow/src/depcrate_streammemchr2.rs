// Generated macro for memchr2 (function)
macro_rules! Depcrate_streammemchr2 {
() => {
// Module: crate::stream
// Provides: {"memchr2"}
// Dependencies: {}
# [cfg (not (feature = "simd"))] # [inline (always)] fn memchr2 (token : (u8 , u8) , slice : & [u8]) -> Option < usize > { slice . iter () . position (| t | * t == token . 0 || * t == token . 1) }
};
}
