// Generated macro for impl_2406 (impl)
macro_rules! Depcrate_path_dec_intimpl_2406 {
() => {
// Module: crate::path::dec_int
// Provides: {"impl_2406"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg (any (not (target_os = "wasi") , not (target_env = "p2") , wasip2))] impl AsRef < Path > for DecInt { # [inline] fn as_ref (& self) -> & Path { let as_os_str : & OsStr = OsStrExt :: from_bytes (self . as_bytes ()) ; Path :: new (as_os_str) } }
};
}
