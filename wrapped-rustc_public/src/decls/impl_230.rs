macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < 'tcx , T > Stable < 'tcx > for & T where T : Stable < 'tcx > , { type T = T :: T ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { (* self) . stable (tables , cx) } }
    };
}

impl_230!();