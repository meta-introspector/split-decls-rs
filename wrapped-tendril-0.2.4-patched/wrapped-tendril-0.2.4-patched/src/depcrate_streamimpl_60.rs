// Generated macro for impl_60 (impl)
macro_rules! Depcrate_streamimpl_60 {
() => {
// Module: crate::stream
// Provides: {"impl_60"}
// Dependencies: {}
impl < Sink , A > TendrilSink < fmt :: Bytes , A > for LossyDecoder < Sink , A > where Sink : TendrilSink < fmt :: UTF8 , A > , A : Atomicity , { # [inline] fn process (& mut self , mut t : Tendril < fmt :: Bytes , A >) { let (decoder , sink) = match self . inner { LossyDecoderInner :: Utf8 (ref mut utf8) => return utf8 . process (t) , LossyDecoderInner :: Other (ref mut decoder , ref mut sink) => (decoder , sink) , } ; let mut out = Tendril :: new () ; loop { match decoder . raw_feed (& * t , & mut out) { (_ , Some (err)) => { out . push_char ('\u{fffd}') ; sink . error (err . cause) ; debug_assert ! (err . upto >= 0) ; t . pop_front (err . upto as u32) ; } (_ , None) => break , } } if out . len () > 0 { sink . process (out) ; } } # [inline] fn error (& mut self , desc : Cow < 'static , str >) { match self . inner { LossyDecoderInner :: Utf8 (ref mut utf8) => utf8 . error (desc) , LossyDecoderInner :: Other (_ , ref mut sink) => sink . error (desc) , } } type Output = Sink :: Output ; # [inline] fn finish (self) -> Sink :: Output { let (mut decoder , mut sink) = match self . inner { LossyDecoderInner :: Utf8 (utf8) => return utf8 . finish () , LossyDecoderInner :: Other (decoder , sink) => (decoder , sink) , } ; let mut out = Tendril :: new () ; if let Some (err) = decoder . raw_finish (& mut out) { out . push_char ('\u{fffd}') ; sink . error (err . cause) ; } if out . len () > 0 { sink . process (out) ; } sink . finish () } }
};
}
