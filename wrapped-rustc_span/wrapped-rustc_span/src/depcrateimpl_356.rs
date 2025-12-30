// Generated macro for impl_356 (impl)
macro_rules! Depcrateimpl_356 {
() => {
// Module: crate
// Provides: {"impl_356"}
// Dependencies: {}
impl SpanEncoder for FileEncoder { fn encode_span (& mut self , span : Span) { let span = span . data () ; span . lo . encode (self) ; span . hi . encode (self) ; } fn encode_symbol (& mut self , sym : Symbol) { self . emit_str (sym . as_str ()) ; } fn encode_byte_symbol (& mut self , byte_sym : ByteSymbol) { self . emit_byte_str (byte_sym . as_byte_str ()) ; } fn encode_expn_id (& mut self , _expn_id : ExpnId) { panic ! ("cannot encode `ExpnId` with `FileEncoder`") ; } fn encode_syntax_context (& mut self , _syntax_context : SyntaxContext) { panic ! ("cannot encode `SyntaxContext` with `FileEncoder`") ; } fn encode_crate_num (& mut self , crate_num : CrateNum) { self . emit_u32 (crate_num . as_u32 ()) ; } fn encode_def_index (& mut self , _def_index : DefIndex) { panic ! ("cannot encode `DefIndex` with `FileEncoder`") ; } fn encode_def_id (& mut self , def_id : DefId) { def_id . krate . encode (self) ; def_id . index . encode (self) ; } }
};
}
