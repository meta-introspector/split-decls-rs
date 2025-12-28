macro_rules! typeid_for_trait_ref {
    () => {
        pub fn typeid_for_trait_ref < 'tcx > (tcx : TyCtxt < 'tcx > , trait_ref : ty :: ExistentialTraitRef < 'tcx > ,) -> String { v0 :: mangle_typeid_for_trait_ref (tcx , trait_ref) }
    };
}

typeid_for_trait_ref!()