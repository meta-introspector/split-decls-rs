// Generated macro for set_executable_file (function)
macro_rules! Depcrate_process_prctlset_executable_file {
() => {
// Module: crate::process::prctl
// Provides: {"set_executable_file"}
// Dependencies: {}
# [doc = " Supersede the `/proc/pid/exe` symbolic link with a new one pointing to a"] # [doc = " new executable file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_MM,PR_SET_MM_EXE_FILE,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_MM,PR_SET_MM_EXE_FILE,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_MM")] # [doc (alias = "PR_SET_MM_EXE_FILE")] pub fn set_executable_file (fd : BorrowedFd < '_ >) -> io :: Result < () > { let fd = usize :: try_from (fd . as_raw_fd ()) . map_err (| _r | io :: Errno :: RANGE) ? ; unsafe { prctl_3args (PR_SET_MM , PR_SET_MM_EXE_FILE as * mut _ , fd as * mut _) } . map (| _r | ()) }
};
}
