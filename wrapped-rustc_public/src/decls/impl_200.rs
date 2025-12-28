macro_rules! deps {
    () => {
        Stable!();
        ClauseKind!();
        BridgeTys!();
        OutlivesPredicate!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: ClauseKind < 'tcx > { type T = crate :: ty :: ClauseKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: ty :: ClauseKind ; match * self { ClauseKind :: Trait (trait_object) => { crate :: ty :: ClauseKind :: Trait (trait_object . stable (tables , cx)) } ClauseKind :: RegionOutlives (region_outlives) => { crate :: ty :: ClauseKind :: RegionOutlives (region_outlives . stable (tables , cx)) } ClauseKind :: TypeOutlives (type_outlives) => { let ty :: OutlivesPredicate :: < _ , _ > (a , b) = type_outlives ; crate :: ty :: ClauseKind :: TypeOutlives (crate :: ty :: OutlivesPredicate (a . stable (tables , cx) , b . stable (tables , cx) ,)) } ClauseKind :: Projection (projection_predicate) => { crate :: ty :: ClauseKind :: Projection (projection_predicate . stable (tables , cx)) } ClauseKind :: ConstArgHasType (const_ , ty) => crate :: ty :: ClauseKind :: ConstArgHasType (const_ . stable (tables , cx) , ty . stable (tables , cx) ,) , ClauseKind :: WellFormed (term) => { crate :: ty :: ClauseKind :: WellFormed (term . kind () . stable (tables , cx)) } ClauseKind :: ConstEvaluatable (const_) => { crate :: ty :: ClauseKind :: ConstEvaluatable (const_ . stable (tables , cx)) } ClauseKind :: HostEffect (..) => { todo ! () } ClauseKind :: UnstableFeature (_) => { todo ! () } } } }
    };
}

impl_200!();