macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        TraitPredicate!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: TraitPredicate < 'tcx > { type T = crate :: ty :: TraitPredicate ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: TraitPredicate { trait_ref , polarity } = self ; crate :: ty :: TraitPredicate { trait_ref : trait_ref . stable (tables , cx) , polarity : polarity . stable (tables , cx) , } } }
    };
}

impl_205!();