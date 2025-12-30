// Generated macro for inline_tag (function)
macro_rules! Depcrate_tendrilinline_tag {
() => {
// Module: crate::tendril
// Provides: {"inline_tag"}
// Dependencies: {}
# [inline (always)] fn inline_tag (len : u32) -> NonZero < usize > { debug_assert ! (len <= MAX_INLINE_LEN as u32) ; unsafe { NonZero :: new (if len == 0 { EMPTY_TAG } else { len as usize }) } }
};
}
