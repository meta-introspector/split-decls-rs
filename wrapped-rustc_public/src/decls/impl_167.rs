macro_rules! deps {
    () => {
        ExistentialPredicate!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: ExistentialPredicate < 'tcx > { type T = crate :: ty :: ExistentialPredicate ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: ExistentialPredicate :: * ; match self { ty :: ExistentialPredicate :: Trait (existential_trait_ref) => { Trait (existential_trait_ref . stable (tables , cx)) } ty :: ExistentialPredicate :: Projection (existential_projection) => { Projection (existential_projection . stable (tables , cx)) } ty :: ExistentialPredicate :: AutoTrait (def_id) => AutoTrait (tables . trait_def (* def_id)) , } } }
    };
}

impl_167!()