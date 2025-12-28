macro_rules! deps {
    () => {
        BridgeTys!();
        SubtypePredicate!();
        Stable!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: SubtypePredicate < 'tcx > { type T = crate :: ty :: SubtypePredicate ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: SubtypePredicate { a , b , a_is_expected : _ } = self ; crate :: ty :: SubtypePredicate { a : a . stable (tables , cx) , b : b . stable (tables , cx) } } }
    };
}

impl_202!()