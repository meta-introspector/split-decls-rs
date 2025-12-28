macro_rules! deps {
    () => {
        TraitRef!();
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: TraitRef < 'tcx > { type T = crate :: ty :: TraitRef ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: TraitRef ; TraitRef :: try_new (tables . trait_def (self . def_id) , self . args . stable (tables , cx)) . unwrap () } }
    };
}

impl_195!()