// Generated macro for fd_fdstat_set_rights (function)
macro_rules! Depcrate_lib_generatedfd_fdstat_set_rights {
() => {
// Module: crate::lib_generated
// Provides: {"fd_fdstat_set_rights"}
// Dependencies: {}
# [doc = " Adjust the rights associated with a file descriptor."] # [doc = " This can only be used to remove rights, and returns `errno::notcapable` if called in a way that would attempt to add rights"] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `fs_rights_base` - The desired rights of the file descriptor."] pub unsafe fn fd_fdstat_set_rights (fd : Fd , fs_rights_base : Rights , fs_rights_inheriting : Rights ,) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: fd_fdstat_set_rights (fd as i32 , fs_rights_base as i64 , fs_rights_inheriting as i64 ,) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
