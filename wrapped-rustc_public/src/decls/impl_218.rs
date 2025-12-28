macro_rules! deps {
    () => {
        AssocContainer!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: AssocContainer { type T = crate :: ty :: AssocContainer ; fn stable (& self , tables : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys > ,) -> Self :: T { use crate :: ty :: AssocContainer ; match self { ty :: AssocContainer :: Trait => AssocContainer :: Trait , ty :: AssocContainer :: InherentImpl => AssocContainer :: InherentImpl , ty :: AssocContainer :: TraitImpl (trait_item_id) => { AssocContainer :: TraitImpl (tables . assoc_def (trait_item_id . unwrap ())) } } } }
    };
}

impl_218!();