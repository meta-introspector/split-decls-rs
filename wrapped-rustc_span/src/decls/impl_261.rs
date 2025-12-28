macro_rules! deps {
    () => {
        SyntaxContext!();
        SpanData!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl Default for SpanData { fn default () -> Self { Self { lo : BytePos (0) , hi : BytePos (0) , ctxt : SyntaxContext :: root () , parent : None } } }
    };
}

impl_261!()