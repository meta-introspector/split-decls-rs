// Generated macro for BufferWriter (struct)
macro_rules! DepcrateBufferWriter {
() => {
// Module: crate
// Provides: {"BufferWriter"}
// Dependencies: {}
# [doc = " Writes colored buffers to stdout or stderr."] # [doc = ""] # [doc = " Writable buffers can be obtained by calling `buffer` on a `BufferWriter`."] # [doc = ""] # [doc = " This writer works with terminals that support ANSI escape sequences or"] # [doc = " with a Windows console."] # [doc = ""] # [doc = " It is intended for a `BufferWriter` to be put in an `Arc` and written to"] # [doc = " from multiple threads simultaneously."] # [derive (Debug)] pub struct BufferWriter { stream : LossyStandardStream < IoStandardStream > , printed : AtomicBool , separator : Option < Vec < u8 > > , color_choice : ColorChoice , # [cfg (windows)] console : Option < Mutex < wincon :: Console > > , }
};
}
