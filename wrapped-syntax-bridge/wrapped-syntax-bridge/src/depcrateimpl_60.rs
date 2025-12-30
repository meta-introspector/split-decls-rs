// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a , Ctx > TtTreeSink < 'a , Ctx > where SpanData < Ctx > : Copy , { fn new (cursor : Cursor < 'a , SpanData < Ctx > >) -> Self { TtTreeSink { buf : String :: new () , cursor , text_pos : 0 . into () , inner : SyntaxTreeBuilder :: default () , token_map : SpanMap :: empty () , } } fn finish (mut self) -> (Parse < SyntaxNode > , SpanMap < Ctx >) { self . token_map . finish () ; (self . inner . finish () , self . token_map) } }
};
}
