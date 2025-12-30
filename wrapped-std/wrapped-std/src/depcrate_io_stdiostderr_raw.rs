// Generated macro for stderr_raw (function)
macro_rules! Depcrate_io_stdiostderr_raw {
() => {
// Module: crate::io::stdio
// Provides: {"stderr_raw"}
// Dependencies: {}
# [doc = " Constructs a new raw handle to the standard error stream of this process."] # [doc = ""] # [doc = " The returned handle does not interact with any other handles created nor"] # [doc = " handles returned by `std::io::stderr`."] # [doc = ""] # [doc = " The returned handle has no external synchronization or buffering layered on"] # [doc = " top."] # [unstable (feature = "libstd_sys_internals" , issue = "none")] const fn stderr_raw () -> StderrRaw { StderrRaw (stdio :: Stderr :: new ()) }
};
}
