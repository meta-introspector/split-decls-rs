// Generated macro for ultostr (function)
macro_rules! Depcrate_unix_apple_macos_component_x86ultostr {
() => {
// Module: crate::unix::apple::macos::component::x86
// Provides: {"ultostr"}
// Dependencies: {}
# [inline] unsafe fn ultostr (s : * mut c_char , val : u32) { unsafe { * s . offset (0) = ((val >> 24) % 128) as i8 ; * s . offset (1) = ((val >> 16) % 128) as i8 ; * s . offset (2) = ((val >> 8) % 128) as i8 ; * s . offset (3) = (val % 128) as i8 ; * s . offset (4) = 0 ; } }
};
}
