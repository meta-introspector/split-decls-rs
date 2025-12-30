// Generated macro for link (function)
macro_rules! Depcrate_os_wasi_fslink {
() => {
// Module: crate::os::wasi::fs
// Provides: {"link"}
// Dependencies: {}
# [doc = " Creates a hard link."] # [doc = ""] # [doc = " This corresponds to the `path_link` syscall."] # [doc (alias = "path_link")] pub fn link < P : AsRef < Path > , U : AsRef < Path > > (old_fd : & File , old_flags : u32 , old_path : P , new_fd : & File , new_path : U ,) -> io :: Result < () > { old_fd . as_inner () . as_inner () . link (old_flags , osstr2str (old_path . as_ref () . as_ref ()) ? , new_fd . as_inner () . as_inner () , osstr2str (new_path . as_ref () . as_ref ()) ? ,) }
};
}
