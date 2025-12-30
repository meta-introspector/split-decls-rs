// Generated macro for BinaryDetection (enum)
macro_rules! Depcrate_line_bufferBinaryDetection {
() => {
// Module: crate::line_buffer
// Provides: {"BinaryDetection"}
// Dependencies: {}
# [doc = " The behavior of binary detection in the line buffer."] # [doc = ""] # [doc = " Binary detection is the process of _heuristically_ identifying whether a"] # [doc = " given chunk of data is binary or not, and then taking an action based on"] # [doc = " the result of that heuristic. The motivation behind detecting binary data"] # [doc = " is that binary data often indicates data that is undesirable to search"] # [doc = " using textual patterns. Of course, there are many cases in which this isn't"] # [doc = " true, which is why binary detection is disabled by default."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub (crate) enum BinaryDetection { # [doc = " No binary detection is performed. Data reported by the line buffer may"] # [doc = " contain arbitrary bytes."] None , # [doc = " The given byte is searched in all contents read by the line buffer. If"] # [doc = " it occurs, then the data is considered binary and the line buffer acts"] # [doc = " as if it reached EOF. The line buffer guarantees that this byte will"] # [doc = " never be observable by callers."] Quit (u8) , # [doc = " The given byte is searched in all contents read by the line buffer. If"] # [doc = " it occurs, then it is replaced by the line terminator. The line buffer"] # [doc = " guarantees that this byte will never be observable by callers."] Convert (u8) , }
};
}
