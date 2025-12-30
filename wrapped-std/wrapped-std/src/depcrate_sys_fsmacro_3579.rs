// Generated macro for macro_3579 (macro)
macro_rules! Depcrate_sys_fsmacro_3579 {
() => {
// Module: crate::sys::fs
// Provides: {"macro_3579"}
// Dependencies: {}
cfg_select ! { target_family = "unix" => { mod unix ; use unix as imp ; pub use unix :: { chown , fchown , lchown , mkfifo } ; # [cfg (not (target_os = "fuchsia"))] pub use unix :: chroot ; pub (crate) use unix :: debug_assert_fd_is_open ; # [cfg (any (target_os = "linux" , target_os = "android"))] pub (crate) use unix :: CachedFileMetadata ; use crate :: sys :: common :: small_c_string :: run_path_with_cstr as with_native_path ; } target_os = "windows" => { mod windows ; use windows as imp ; pub use windows :: { symlink_inner , junction_point } ; use crate :: sys :: path :: with_native_path ; } target_os = "hermit" => { mod hermit ; use hermit as imp ; } target_os = "solid_asp3" => { mod solid ; use solid as imp ; } target_os = "uefi" => { mod uefi ; use uefi as imp ; } target_os = "wasi" => { mod wasi ; use wasi as imp ; } _ => { mod unsupported ; use unsupported as imp ; } }
};
}
