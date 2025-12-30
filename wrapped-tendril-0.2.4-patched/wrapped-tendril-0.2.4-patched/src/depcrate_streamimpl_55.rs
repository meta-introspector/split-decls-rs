// Generated macro for impl_55 (impl)
macro_rules! Depcrate_streamimpl_55 {
() => {
// Module: crate::stream
// Provides: {"impl_55"}
// Dependencies: {}
impl < Sink , A > Utf8LossyDecoder < Sink , A > where Sink : TendrilSink < fmt :: UTF8 , A > , A : Atomicity , { # [doc = " Create a new incremental UTF-8 decoder."] # [inline] pub fn new (inner_sink : Sink) -> Self { Utf8LossyDecoder { decoder : utf8 :: Decoder :: new () , inner_sink : inner_sink , marker : PhantomData , } } }
};
}
