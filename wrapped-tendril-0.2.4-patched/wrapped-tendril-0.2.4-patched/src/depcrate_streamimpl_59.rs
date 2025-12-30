// Generated macro for impl_59 (impl)
macro_rules! Depcrate_streamimpl_59 {
() => {
// Module: crate::stream
// Provides: {"impl_59"}
// Dependencies: {}
impl < Sink , A > LossyDecoder < Sink , A > where Sink : TendrilSink < fmt :: UTF8 , A > , A : Atomicity , { # [doc = " Create a new incremental decoder."] # [inline] pub fn new (encoding : EncodingRef , sink : Sink) -> LossyDecoder < Sink , A > { if encoding . name () == "utf-8" { LossyDecoder :: utf8 (sink) } else { LossyDecoder { inner : LossyDecoderInner :: Other (encoding . raw_decoder () , sink) } } } # [doc = " Create a new incremental decoder for the UTF-8 encoding."] # [doc = ""] # [doc = " This is useful for content that is known at run-time to be UTF-8"] # [doc = " (whereas `Utf8LossyDecoder` requires knowning at compile-time.)"] # [inline] pub fn utf8 (sink : Sink) -> LossyDecoder < Sink , A > { LossyDecoder { inner : LossyDecoderInner :: Utf8 (Utf8LossyDecoder :: new (sink)) } } # [doc = " Give a reference to the inner sink."] pub fn inner_sink (& self) -> & Sink { match self . inner { LossyDecoderInner :: Utf8 (ref utf8) => & utf8 . inner_sink , LossyDecoderInner :: Other (_ , ref inner_sink) => inner_sink , } } # [doc = " Give a mutable reference to the inner sink."] pub fn inner_sink_mut (& mut self) -> & mut Sink { match self . inner { LossyDecoderInner :: Utf8 (ref mut utf8) => & mut utf8 . inner_sink , LossyDecoderInner :: Other (_ , ref mut inner_sink) => inner_sink , } } }
};
}
