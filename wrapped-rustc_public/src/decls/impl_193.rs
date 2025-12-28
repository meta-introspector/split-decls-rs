macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        TraitSpecializationKind!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: trait_def :: TraitSpecializationKind { type T = crate :: ty :: TraitSpecializationKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use crate :: ty :: TraitSpecializationKind ; match self { ty :: trait_def :: TraitSpecializationKind :: None => TraitSpecializationKind :: None , ty :: trait_def :: TraitSpecializationKind :: Marker => TraitSpecializationKind :: Marker , ty :: trait_def :: TraitSpecializationKind :: AlwaysApplicable => { TraitSpecializationKind :: AlwaysApplicable } } } }
    };
}

impl_193!();