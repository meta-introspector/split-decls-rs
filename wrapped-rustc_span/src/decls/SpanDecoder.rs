macro_rules! deps {
    () => {
        Span!();
        ExpnId!();
        ByteSymbol!();
        SyntaxContext!();
        DefId!();
        Symbol!();
    };
}

macro_rules! SpanDecoder {
    () => {
        deps!();
        # [doc = " This trait is used to allow decoder specific encodings of certain types."] # [doc = " It is similar to rustc_type_ir's TyDecoder."] pub trait SpanDecoder : Decoder { fn decode_span (& mut self) -> Span ; fn decode_symbol (& mut self) -> Symbol ; fn decode_byte_symbol (& mut self) -> ByteSymbol ; fn decode_expn_id (& mut self) -> ExpnId ; fn decode_syntax_context (& mut self) -> SyntaxContext ; fn decode_crate_num (& mut self) -> CrateNum ; fn decode_def_index (& mut self) -> DefIndex ; fn decode_def_id (& mut self) -> DefId ; fn decode_attr_id (& mut self) -> AttrId ; }
    };
}

SpanDecoder!();