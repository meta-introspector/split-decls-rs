// Generated macro for memchr (function)
macro_rules! Depcrate_streammemchr {
() => {
// Module: crate::stream
// Provides: {"memchr"}
// Dependencies: {}
# [cfg (not (feature = "simd"))] # [inline (always)] fn memchr (token : u8 , slice : & [u8]) -> Option < usize > { slice . iter () . position (| t | * t == token) }
};
}
