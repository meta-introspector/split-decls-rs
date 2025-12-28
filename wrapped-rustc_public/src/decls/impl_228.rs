macro_rules! deps {
    () => {
        Stable!();
        Span!();
        BridgeTys!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_span :: Span { type T = crate :: ty :: Span ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , _ : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { tables . create_span (* self) } }
    };
}

impl_228!()