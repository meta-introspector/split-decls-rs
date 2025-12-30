// Generated macro for alloc_error (function)
macro_rules! Depcrate_line_bufferalloc_error {
() => {
// Module: crate::line_buffer
// Provides: {"alloc_error"}
// Dependencies: {}
# [doc = " Create a new error to be used when a configured allocation limit has been"] # [doc = " reached."] pub (crate) fn alloc_error (limit : usize) -> io :: Error { let msg = format ! ("configured allocation limit ({}) exceeded" , limit) ; io :: Error :: new (io :: ErrorKind :: Other , msg) }
};
}
