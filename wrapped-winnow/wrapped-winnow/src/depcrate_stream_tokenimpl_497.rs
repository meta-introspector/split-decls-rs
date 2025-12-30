// Generated macro for impl_497 (impl)
macro_rules! Depcrate_stream_tokenimpl_497 {
() => {
// Module: crate::stream::token
// Provides: {"impl_497"}
// Dependencies: {}
impl < 't , T > TokenSlice < 't , T > where T : core :: fmt :: Debug + Clone , { # [doc = " Make a stream to parse tokens"] # [inline] pub fn new (input : & 't [T]) -> Self { Self { initial : input , input , } } # [doc = " Reset the stream to the start"] # [doc = ""] # [doc = " This is useful for formats that encode a graph with addresses relative to the start of the"] # [doc = " input."] # [doc (alias = "fseek")] # [inline] pub fn reset_to_start (& mut self) { let start = self . initial . checkpoint () ; self . input . reset (& start) ; } # [doc = " Iterate over consumed tokens starting with the last emitted"] # [doc = ""] # [doc = " This is intended to help build up appropriate context when reporting errors."] # [inline] pub fn previous_tokens (& self) -> impl Iterator < Item = & 't T > { let offset = self . input . offset_from (& self . initial) ; self . initial [0 .. offset] . iter () . rev () } }
};
}
