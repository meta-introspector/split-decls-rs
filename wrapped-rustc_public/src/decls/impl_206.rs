macro_rules! deps {
    () => {
        Stable!();
        OutlivesPredicate!();
        Region!();
        BridgeTys!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < 'tcx , T > Stable < 'tcx > for ty :: OutlivesPredicate < 'tcx , T > where T : Stable < 'tcx > , { type T = crate :: ty :: OutlivesPredicate < T :: T , Region > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: OutlivesPredicate (a , b) = self ; crate :: ty :: OutlivesPredicate (a . stable (tables , cx) , b . stable (tables , cx)) } }
    };
}

impl_206!();