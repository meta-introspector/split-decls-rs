macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: Statement < 'tcx > { type T = crate :: mir :: Statement ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { Statement { kind : self . kind . stable (tables , cx) , span : self . source_info . span . stable (tables , cx) , } } }
    };
}

impl_125!();