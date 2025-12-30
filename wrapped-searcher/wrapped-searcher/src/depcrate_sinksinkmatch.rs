// Generated macro for SinkMatch (struct)
macro_rules! Depcrate_sinkSinkMatch {
() => {
// Module: crate::sink
// Provides: {"SinkMatch"}
// Dependencies: {}
# [doc = " A type that describes a match reported by a searcher."] # [derive (Clone , Debug)] pub struct SinkMatch < 'b > { pub (crate) line_term : LineTerminator , pub (crate) bytes : & 'b [u8] , pub (crate) absolute_byte_offset : u64 , pub (crate) line_number : Option < u64 > , pub (crate) buffer : & 'b [u8] , pub (crate) bytes_range_in_buffer : std :: ops :: Range < usize > , }
};
}
