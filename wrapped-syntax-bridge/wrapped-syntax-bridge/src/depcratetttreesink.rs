// Generated macro for TtTreeSink (struct)
macro_rules! DepcrateTtTreeSink {
() => {
// Module: crate
// Provides: {"TtTreeSink"}
// Dependencies: {}
struct TtTreeSink < 'a , Ctx > where SpanData < Ctx > : Copy , { buf : String , cursor : Cursor < 'a , SpanData < Ctx > > , text_pos : TextSize , inner : SyntaxTreeBuilder , token_map : SpanMap < Ctx > , }
};
}
