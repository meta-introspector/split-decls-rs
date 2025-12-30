// Generated macro for Buffer (struct)
macro_rules! DepcrateBuffer {
() => {
// Module: crate
// Provides: {"Buffer"}
// Dependencies: {}
# [doc = " Write colored text to memory."] # [doc = ""] # [doc = " `Buffer` is a platform independent abstraction for printing colored text to"] # [doc = " an in memory buffer. When the buffer is printed using a `BufferWriter`, the"] # [doc = " color information will be applied to the output device (a tty on Unix and a"] # [doc = " console on Windows)."] # [doc = ""] # [doc = " A `Buffer` is typically created by calling the `BufferWriter.buffer`"] # [doc = " method, which will take color preferences and the environment into"] # [doc = " account. However, buffers can also be manually created using `no_color`,"] # [doc = " `ansi` or `console` (on Windows)."] # [derive (Clone , Debug)] pub struct Buffer (BufferInner) ;
};
}
