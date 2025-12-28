macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < 'tcx , T > Stable < 'tcx > for Option < T > where T : Stable < 'tcx > , { type T = Option < T :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { self . as_ref () . map (| value | value . stable (tables , cx)) } }
    };
}

impl_231!();