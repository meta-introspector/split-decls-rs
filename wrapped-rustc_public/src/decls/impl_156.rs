macro_rules! deps {
    () => {
        Size!();
        Allocation!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: interpret :: Allocation { type T = crate :: ty :: Allocation ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_public_bridge :: context :: AllocRangeHelpers ; alloc :: allocation_filter (self , cx . alloc_range (rustc_abi :: Size :: ZERO , self . size ()) , tables , cx ,) } }
    };
}

impl_156!()