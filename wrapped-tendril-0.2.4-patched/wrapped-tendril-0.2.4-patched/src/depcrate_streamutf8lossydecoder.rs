// Generated macro for Utf8LossyDecoder (struct)
macro_rules! Depcrate_streamUtf8LossyDecoder {
() => {
// Module: crate::stream
// Provides: {"Utf8LossyDecoder"}
// Dependencies: {}
# [doc = " A `TendrilSink` adaptor that takes bytes, decodes them as UTF-8,"] # [doc = " lossily replace ill-formed byte sequences with U+FFFD replacement characters,"] # [doc = " and emits Unicode (`StrTendril`)."] # [doc = ""] # [doc = " This does not allocate memory: the output is either subtendrils on the input,"] # [doc = " on inline tendrils for a single code point."] pub struct Utf8LossyDecoder < Sink , A = NonAtomic > where Sink : TendrilSink < fmt :: UTF8 , A > , A : Atomicity { decoder : utf8 :: Decoder , pub inner_sink : Sink , marker : PhantomData < A > , }
};
}
