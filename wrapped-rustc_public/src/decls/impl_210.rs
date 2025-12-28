macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        Region!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: Region < 'tcx > { type T = crate :: ty :: Region ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { Region { kind : self . kind () . stable (tables , cx) } } }
    };
}

impl_210!();