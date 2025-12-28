macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: SourceInfo { type T = crate :: mir :: SourceInfo ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: mir :: SourceInfo { span : self . span . stable (tables , cx) , scope : self . scope . into () } } }
    };
}

impl_126!();