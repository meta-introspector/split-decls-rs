macro_rules! deps {
    () => {
        ClauseKind!();
        Ty!();
        Component!();
        Region!();
        OutlivesPredicate!();
        Interner!();
    };
}

macro_rules! elaborate_component_to_clause {
    () => {
        deps!();
        fn elaborate_component_to_clause < I : Interner > (cx : I , component : Component < I > , outlives_region : I :: Region ,) -> Option < ty :: ClauseKind < I > > { match component { Component :: Region (r) => { if r . is_bound () { None } else { Some (ty :: ClauseKind :: RegionOutlives (ty :: OutlivesPredicate (r , outlives_region))) } } Component :: Param (p) => { let ty = Ty :: new_param (cx , p) ; Some (ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (ty , outlives_region))) } Component :: Placeholder (p) => { let ty = Ty :: new_placeholder (cx , p) ; Some (ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (ty , outlives_region))) } Component :: UnresolvedInferenceVariable (_) => None , Component :: Alias (alias_ty) => { Some (ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (alias_ty . to_ty (cx) , outlives_region ,))) } Component :: EscapingAlias (_) => { None } } }
    };
}

elaborate_component_to_clause!()