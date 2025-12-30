// Generated macro for impl_336 (impl)
macro_rules! Depcrate_stream_locatingimpl_336 {
() => {
// Module: crate::stream::locating
// Provides: {"impl_336"}
// Dependencies: {}
impl < I > LocatingSlice < I > where I : Clone + Stream + Offset , { # [doc = " Reset the stream to the start"] # [doc = ""] # [doc = " This is useful for formats that encode a graph with addresses relative to the start of the"] # [doc = " input."] # [doc (alias = "fseek")] # [inline] pub fn reset_to_start (& mut self) { let start = self . initial . checkpoint () ; self . input . reset (& start) ; } }
};
}
