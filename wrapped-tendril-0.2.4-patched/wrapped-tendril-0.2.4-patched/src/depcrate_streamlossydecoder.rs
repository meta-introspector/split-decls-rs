// Generated macro for LossyDecoder (struct)
macro_rules! Depcrate_streamLossyDecoder {
() => {
// Module: crate::stream
// Provides: {"LossyDecoder"}
// Dependencies: {}
# [doc = " A `TendrilSink` adaptor that takes bytes, decodes them as the given character encoding,"] # [doc = " lossily replace ill-formed byte sequences with U+FFFD replacement characters,"] # [doc = " and emits Unicode (`StrTendril`)."] # [doc = ""] # [doc = " This allocates new tendrils for encodings other than UTF-8."] pub struct LossyDecoder < Sink , A = NonAtomic > where Sink : TendrilSink < fmt :: UTF8 , A > , A : Atomicity { inner : LossyDecoderInner < Sink , A > , }
};
}
