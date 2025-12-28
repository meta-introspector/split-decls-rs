macro_rules! deps {
    () => {
        FindMin!();
    };
}

macro_rules! VisibilityLike {
    () => {
        deps!();
        trait VisibilityLike : Sized { const MAX : Self ; fn new_min < const SHALLOW : bool > (find : & FindMin < '_ , '_ , Self , SHALLOW > , def_id : LocalDefId ,) -> Self ; fn of_impl < const SHALLOW : bool > (def_id : LocalDefId , tcx : TyCtxt < '_ > , effective_visibilities : & EffectiveVisibilities ,) -> Self { let mut find = FindMin :: < _ , SHALLOW > { tcx , effective_visibilities , min : Self :: MAX } ; find . visit (tcx . type_of (def_id) . instantiate_identity ()) ; if let Some (trait_ref) = tcx . impl_trait_ref (def_id) { find . visit_trait (trait_ref . instantiate_identity ()) ; } find . min } }
    };
}

VisibilityLike!()