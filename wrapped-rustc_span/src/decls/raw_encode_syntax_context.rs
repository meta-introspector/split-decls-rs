macro_rules! deps {
    () => {
        SyntaxContext!();
        HygieneEncodeContext!();
    };
}

macro_rules! raw_encode_syntax_context {
    () => {
        deps!();
        pub fn raw_encode_syntax_context (ctxt : SyntaxContext , context : & HygieneEncodeContext , e : & mut impl Encoder ,) { if ! context . serialized_ctxts . lock () . contains (& ctxt) { context . latest_ctxts . lock () . insert (ctxt) ; } ctxt . 0 . encode (e) ; }
    };
}

raw_encode_syntax_context!();