macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        ImplTraitInTraitData!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: ImplTraitInTraitData { type T = crate :: ty :: ImplTraitInTraitData ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , _ : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: ImplTraitInTraitData ; match self { ty :: ImplTraitInTraitData :: Trait { fn_def_id , opaque_def_id } => { ImplTraitInTraitData :: Trait { fn_def_id : tables . fn_def (* fn_def_id) , opaque_def_id : tables . opaque_def (* opaque_def_id) , } } ty :: ImplTraitInTraitData :: Impl { fn_def_id } => { ImplTraitInTraitData :: Impl { fn_def_id : tables . fn_def (* fn_def_id) } } } } }
    };
}

impl_220!();