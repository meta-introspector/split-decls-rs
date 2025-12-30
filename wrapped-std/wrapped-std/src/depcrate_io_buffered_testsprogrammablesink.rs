// Generated macro for ProgrammableSink (struct)
macro_rules! Depcrate_io_buffered_testsProgrammableSink {
() => {
// Module: crate::io::buffered::tests
// Provides: {"ProgrammableSink"}
// Dependencies: {}
# [doc = " A simple `Write` target, designed to be wrapped by `LineWriter` /"] # [doc = " `BufWriter` / etc, that can have its `write` & `flush` behavior"] # [doc = " configured"] # [derive (Default , Clone)] struct ProgrammableSink { pub buffer : Vec < u8 > , pub always_write_error : bool , pub always_flush_error : bool , pub accept_prefix : Option < usize > , pub max_writes : Option < usize > , pub error_after_max_writes : bool , }
};
}
