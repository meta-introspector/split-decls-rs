// Generated macro for LineWriterShim (struct)
macro_rules! Depcrate_io_buffered_linewritershimLineWriterShim {
() => {
// Module: crate::io::buffered::linewritershim
// Provides: {"LineWriterShim"}
// Dependencies: {}
# [doc = " Private helper struct for implementing the line-buffered writing logic."] # [doc = ""] # [doc = " This shim temporarily wraps a BufWriter, and uses its internals to"] # [doc = " implement a line-buffered writer (specifically by using the internal"] # [doc = " methods like write_to_buf and flush_buf). In this way, a more"] # [doc = " efficient abstraction can be created than one that only had access to"] # [doc = " `write` and `flush`, without needlessly duplicating a lot of the"] # [doc = " implementation details of BufWriter. This also allows existing"] # [doc = " `BufWriters` to be temporarily given line-buffering logic; this is what"] # [doc = " enables Stdout to be alternately in line-buffered or block-buffered mode."] # [derive (Debug)] pub struct LineWriterShim < 'a , W : ? Sized + Write > { buffer : & 'a mut BufWriter < W > , }
};
}
