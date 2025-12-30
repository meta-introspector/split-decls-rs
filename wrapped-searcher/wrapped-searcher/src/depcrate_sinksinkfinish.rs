// Generated macro for SinkFinish (struct)
macro_rules! Depcrate_sinkSinkFinish {
() => {
// Module: crate::sink
// Provides: {"SinkFinish"}
// Dependencies: {}
# [doc = " Summary data reported at the end of a search."] # [doc = ""] # [doc = " This reports data such as the total number of bytes searched and the"] # [doc = " absolute offset of the first occurrence of binary data, if any were found."] # [doc = ""] # [doc = " A searcher that stops early because of an error does not call `finish`."] # [doc = " A searcher that stops early because the `Sink` implementor instructed it"] # [doc = " to will still call `finish`."] # [derive (Clone , Debug)] pub struct SinkFinish { pub (crate) byte_count : u64 , pub (crate) binary_byte_offset : Option < u64 > , }
};
}
