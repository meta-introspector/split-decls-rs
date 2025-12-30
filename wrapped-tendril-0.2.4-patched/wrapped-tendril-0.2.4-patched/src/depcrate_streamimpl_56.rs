// Generated macro for impl_56 (impl)
macro_rules! Depcrate_streamimpl_56 {
() => {
// Module: crate::stream
// Provides: {"impl_56"}
// Dependencies: {}
impl < Sink , A > TendrilSink < fmt :: Bytes , A > for Utf8LossyDecoder < Sink , A > where Sink : TendrilSink < fmt :: UTF8 , A > , A : Atomicity , { # [inline] fn process (& mut self , t : Tendril < fmt :: Bytes , A >) { let mut input = & * t ; loop { let (ch , s , result) = self . decoder . decode (input) ; if ! ch . is_empty () { self . inner_sink . process (Tendril :: from_slice (& * ch)) ; } if ! s . is_empty () { let offset = s . as_ptr () as usize - t . as_ptr () as usize ; let subtendril = t . subtendril (offset as u32 , s . len () as u32) ; unsafe { self . inner_sink . process (subtendril . reinterpret_without_validating ()) ; } } match result { utf8 :: Result :: Ok | utf8 :: Result :: Incomplete => break , utf8 :: Result :: Error { remaining_input_after_error : remaining } => { self . inner_sink . error ("invalid byte sequence" . into ()) ; self . inner_sink . process (Tendril :: from_slice (utf8 :: REPLACEMENT_CHARACTER)) ; input = remaining ; } } } } # [inline] fn error (& mut self , desc : Cow < 'static , str >) { self . inner_sink . error (desc) ; } type Output = Sink :: Output ; # [inline] fn finish (mut self) -> Sink :: Output { if self . decoder . has_incomplete_sequence () { self . inner_sink . error ("incomplete byte sequence at end of stream" . into ()) ; self . inner_sink . process (Tendril :: from_slice (utf8 :: REPLACEMENT_CHARACTER)) ; } self . inner_sink . finish () } }
};
}
