// Generated macro for LineBufferReader (struct)
macro_rules! Depcrate_line_bufferLineBufferReader {
() => {
// Module: crate::line_buffer
// Provides: {"LineBufferReader"}
// Dependencies: {}
# [doc = " A line buffer reader efficiently reads a line oriented buffer from an"] # [doc = " arbitrary reader."] # [derive (Debug)] pub (crate) struct LineBufferReader < 'b , R > { rdr : R , line_buffer : & 'b mut LineBuffer , }
};
}
