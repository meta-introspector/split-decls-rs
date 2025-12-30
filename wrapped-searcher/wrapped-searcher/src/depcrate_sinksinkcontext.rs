// Generated macro for SinkContext (struct)
macro_rules! Depcrate_sinkSinkContext {
() => {
// Module: crate::sink
// Provides: {"SinkContext"}
// Dependencies: {}
# [doc = " A type that describes a contextual line reported by a searcher."] # [derive (Clone , Debug)] pub struct SinkContext < 'b > { # [cfg (test)] pub (crate) line_term : LineTerminator , pub (crate) bytes : & 'b [u8] , pub (crate) kind : SinkContextKind , pub (crate) absolute_byte_offset : u64 , pub (crate) line_number : Option < u64 > , }
};
}
