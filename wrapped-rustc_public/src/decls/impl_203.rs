macro_rules! deps {
    () => {
        CoercePredicate!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: CoercePredicate < 'tcx > { type T = crate :: ty :: CoercePredicate ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: CoercePredicate { a , b } = self ; crate :: ty :: CoercePredicate { a : a . stable (tables , cx) , b : b . stable (tables , cx) } } }
    };
}

impl_203!();