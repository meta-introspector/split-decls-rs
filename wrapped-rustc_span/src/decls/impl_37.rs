macro_rules! deps {
    () => {
        SpanData!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl Default for SpanData { fn default () -> Self { Self { lo : BytePos (0) , hi : BytePos (0) , ctxt : SyntaxContext :: root () , parent : None } } }
    };
}

impl_37!()