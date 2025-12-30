// Generated macro for strtoul (function)
macro_rules! Depcrate_unix_apple_macos_component_x86strtoul {
() => {
// Module: crate::unix::apple::macos::component::x86
// Provides: {"strtoul"}
// Dependencies: {}
# [inline] fn strtoul (s : & [i8]) -> u32 { unsafe { ((* s . get_unchecked (0) as u32) << (3u32 << 3)) + ((* s . get_unchecked (1) as u32) << (2u32 << 3)) + ((* s . get_unchecked (2) as u32) << (1u32 << 3)) + (* s . get_unchecked (3) as u32) } }
};
}
