// Generated macro for macro_3610 (macro)
macro_rules! Depcrate_sys_os_strmacro_3610 {
() => {
// Module: crate::sys::os_str
// Provides: {"macro_3610"}
// Dependencies: {}
cfg_select ! { any (target_os = "windows" , target_os = "uefi") => { mod wtf8 ; pub use wtf8 :: { Buf , Slice } ; } _ => { mod bytes ; pub use bytes :: { Buf , Slice } ; } }
};
}
