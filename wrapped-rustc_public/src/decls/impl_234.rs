macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < 'tcx , T , U > Stable < 'tcx > for (T , U) where T : Stable < 'tcx > , U : Stable < 'tcx > , { type T = (T :: T , U :: T) ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { (self . 0 . stable (tables , cx) , self . 1 . stable (tables , cx)) } }
    };
}

impl_234!()