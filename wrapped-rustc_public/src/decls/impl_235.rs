macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl < 'tcx , T > Stable < 'tcx > for RangeInclusive < T > where T : Stable < 'tcx > , { type T = RangeInclusive < T :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { RangeInclusive :: new (self . start () . stable (tables , cx) , self . end () . stable (tables , cx)) } }
    };
}

impl_235!();