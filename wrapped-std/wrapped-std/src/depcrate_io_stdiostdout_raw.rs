// Generated macro for stdout_raw (function)
macro_rules! Depcrate_io_stdiostdout_raw {
() => {
// Module: crate::io::stdio
// Provides: {"stdout_raw"}
// Dependencies: {}
# [doc = " Constructs a new raw handle to the standard output stream of this process."] # [doc = ""] # [doc = " The returned handle does not interact with any other handles created nor"] # [doc = " handles returned by `std::io::stdout`. Note that data is buffered by the"] # [doc = " `std::io::stdout` handles so writes which happen via this raw handle may"] # [doc = " appear before previous writes."] # [doc = ""] # [doc = " The returned handle has no external synchronization or buffering layered on"] # [doc = " top."] # [unstable (feature = "libstd_sys_internals" , issue = "none")] const fn stdout_raw () -> StdoutRaw { StdoutRaw (stdio :: Stdout :: new ()) }
};
}
