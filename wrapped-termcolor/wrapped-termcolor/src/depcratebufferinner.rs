// Generated macro for BufferInner (enum)
macro_rules! DepcrateBufferInner {
() => {
// Module: crate
// Provides: {"BufferInner"}
// Dependencies: {}
# [doc = " BufferInner is an enumeration of different buffer types."] # [derive (Clone , Debug)] enum BufferInner { # [doc = " No coloring information should be applied. This ignores all coloring"] # [doc = " directives."] NoColor (NoColor < Vec < u8 > >) , # [doc = " Apply coloring using ANSI escape sequences embedded into the buffer."] Ansi (Ansi < Vec < u8 > >) , # [doc = " Apply coloring using the Windows console APIs. This buffer saves"] # [doc = " color information in memory and only interacts with the console when"] # [doc = " the buffer is printed."] # [cfg (windows)] Windows (WindowsBuffer) , }
};
}
