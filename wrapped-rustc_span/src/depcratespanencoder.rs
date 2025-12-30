// Generated macro for SpanEncoder (trait)
macro_rules! DepcrateSpanEncoder {
() => {
// Module: crate
// Provides: {"SpanEncoder"}
// Dependencies: {}
# [doc = " This trait is used to allow encoder specific encodings of certain types."] # [doc = " It is similar to rustc_type_ir's TyEncoder."] pub trait SpanEncoder : Encoder { fn encode_span (& mut self , span : Span) ; fn encode_symbol (& mut self , sym : Symbol) ; fn encode_byte_symbol (& mut self , byte_sym : ByteSymbol) ; fn encode_expn_id (& mut self , expn_id : ExpnId) ; fn encode_syntax_context (& mut self , syntax_context : SyntaxContext) ; # [doc = " As a local identifier, a `CrateNum` is only meaningful within its context, e.g. within a"] # [doc = " tcx. Therefore, make sure to include the context when encode a `CrateNum`."] fn encode_crate_num (& mut self , crate_num : CrateNum) ; fn encode_def_index (& mut self , def_index : DefIndex) ; fn encode_def_id (& mut self , def_id : DefId) ; }
};
}
