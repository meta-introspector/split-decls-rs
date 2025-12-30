// Generated macro for Config (struct)
macro_rules! Depcrate_line_bufferConfig {
() => {
// Module: crate::line_buffer
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The configuration of a buffer. This contains options that are fixed once"] # [doc = " a buffer has been constructed."] # [derive (Clone , Copy , Debug)] struct Config { # [doc = " The number of bytes to attempt to read at a time."] capacity : usize , # [doc = " The line terminator."] lineterm : u8 , # [doc = " The behavior for handling long lines."] buffer_alloc : BufferAllocation , # [doc = " When set, the presence of the given byte indicates binary content."] binary : BinaryDetection , }
};
}
