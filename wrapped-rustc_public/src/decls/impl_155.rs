macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        Allocation!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: interpret :: ConstAllocation < 'tcx > { type T = Allocation ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { self . inner () . stable (tables , cx) } }
    };
}

impl_155!()