macro_rules! deps {
    () => {
        Stable!();
        ProjectionPredicate!();
        BridgeTys!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: ProjectionPredicate < 'tcx > { type T = crate :: ty :: ProjectionPredicate ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: ProjectionPredicate { projection_term , term } = self ; crate :: ty :: ProjectionPredicate { projection_term : projection_term . stable (tables , cx) , term : term . kind () . stable (tables , cx) , } } }
    };
}

impl_207!();