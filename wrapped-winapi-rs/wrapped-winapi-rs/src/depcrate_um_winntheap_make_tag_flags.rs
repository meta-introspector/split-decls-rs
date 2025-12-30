// Generated macro for HEAP_MAKE_TAG_FLAGS (function)
macro_rules! Depcrate_um_winntHEAP_MAKE_TAG_FLAGS {
() => {
// Module: crate::um::winnt
// Provides: {"HEAP_MAKE_TAG_FLAGS"}
// Dependencies: {}
# [inline] pub fn HEAP_MAKE_TAG_FLAGS (TagBase : DWORD , Tag : DWORD) -> DWORD { TagBase + (Tag << HEAP_TAG_SHIFT) }
};
}
