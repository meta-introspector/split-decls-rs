// Generated macro for LossyDecoderInner (enum)
macro_rules! Depcrate_streamLossyDecoderInner {
() => {
// Module: crate::stream
// Provides: {"LossyDecoderInner"}
// Dependencies: {}
enum LossyDecoderInner < Sink , A > where Sink : TendrilSink < fmt :: UTF8 , A > , A : Atomicity { Utf8 (Utf8LossyDecoder < Sink , A >) , Other (Box < RawDecoder > , Sink) }
};
}
