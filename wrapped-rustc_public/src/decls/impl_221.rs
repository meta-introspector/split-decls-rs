macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        Discr!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_middle :: ty :: util :: Discr < 'tcx > { type T = crate :: ty :: Discr ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: ty :: Discr { val : self . val , ty : self . ty . stable (tables , cx) } } }
    };
}

impl_221!();