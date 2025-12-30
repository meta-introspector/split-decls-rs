// Generated macro for impl_101 (impl)
macro_rules! Depcrate_sinkimpl_101 {
() => {
// Module: crate::sink
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'b > SinkContext < 'b > { # [doc = " Returns the context bytes, including line terminators."] # [inline] pub fn bytes (& self) -> & 'b [u8] { self . bytes } # [doc = " Returns the type of context."] # [inline] pub fn kind (& self) -> & SinkContextKind { & self . kind } # [doc = " Return an iterator over the lines in this match."] # [doc = ""] # [doc = " This always yields exactly one line (and that one line may contain just"] # [doc = " the line terminator)."] # [doc = ""] # [doc = " Lines yielded by this iterator include their terminators."] # [cfg (test)] pub (crate) fn lines (& self) -> LineIter < 'b > { LineIter :: new (self . line_term . as_byte () , self . bytes) } # [doc = " Returns the absolute byte offset of the start of this context. This"] # [doc = " offset is absolute in that it is relative to the very beginning of the"] # [doc = " input in a search, and can never be relied upon to be a valid index"] # [doc = " into an in-memory slice."] # [inline] pub fn absolute_byte_offset (& self) -> u64 { self . absolute_byte_offset } # [doc = " Returns the line number of the first line in this context, if"] # [doc = " available."] # [doc = ""] # [doc = " Line numbers are only available when the search builder is instructed"] # [doc = " to compute them."] # [inline] pub fn line_number (& self) -> Option < u64 > { self . line_number } }
};
}
