macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        ExistentialTraitRef!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: ExistentialTraitRef < 'tcx > { type T = crate :: ty :: ExistentialTraitRef ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: ExistentialTraitRef { def_id , args , .. } = self ; crate :: ty :: ExistentialTraitRef { def_id : tables . trait_def (* def_id) , generic_args : args . stable (tables , cx) , } } }
    };
}

impl_168!();