// Generated macro for stdin_raw (function)
macro_rules! Depcrate_io_stdiostdin_raw {
() => {
// Module: crate::io::stdio
// Provides: {"stdin_raw"}
// Dependencies: {}
# [doc = " Constructs a new raw handle to the standard input of this process."] # [doc = ""] # [doc = " The returned handle does not interact with any other handles created nor"] # [doc = " handles returned by `std::io::stdin`. Data buffered by the `std::io::stdin`"] # [doc = " handles is **not** available to raw handles returned from this function."] # [doc = ""] # [doc = " The returned handle has no external synchronization or buffering."] # [unstable (feature = "libstd_sys_internals" , issue = "none")] const fn stdin_raw () -> StdinRaw { StdinRaw (stdio :: Stdin :: new ()) }
};
}
