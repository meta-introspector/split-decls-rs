// Generated macro for BufferAllocation (enum)
macro_rules! Depcrate_line_bufferBufferAllocation {
() => {
// Module: crate::line_buffer
// Provides: {"BufferAllocation"}
// Dependencies: {}
# [doc = " The behavior of a searcher in the face of long lines and big contexts."] # [doc = ""] # [doc = " When searching data incrementally using a fixed size buffer, this controls"] # [doc = " the amount of *additional* memory to allocate beyond the size of the buffer"] # [doc = " to accommodate lines (which may include the lines in a context window, when"] # [doc = " enabled) that do not fit in the buffer."] # [doc = ""] # [doc = " The default is to eagerly allocate without a limit."] # [derive (Clone , Copy , Debug)] pub (crate) enum BufferAllocation { # [doc = " Attempt to expand the size of the buffer until either at least the next"] # [doc = " line fits into memory or until all available memory is exhausted."] # [doc = ""] # [doc = " This is the default."] Eager , # [doc = " Limit the amount of additional memory allocated to the given size. If"] # [doc = " a line is found that requires more memory than is allowed here, then"] # [doc = " stop reading and return an error."] Error (usize) , }
};
}
