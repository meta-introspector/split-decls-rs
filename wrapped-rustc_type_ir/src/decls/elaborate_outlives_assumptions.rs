macro_rules! deps {
    () => {
        GenericArg!();
        Component!();
        Ty!();
        Interner!();
        Const!();
        OutlivesPredicate!();
        GenericArgKind!();
        Region!();
    };
}

macro_rules! elaborate_outlives_assumptions {
    () => {
        deps!();
        pub fn elaborate_outlives_assumptions < I : Interner > (cx : I , assumptions : impl IntoIterator < Item = ty :: OutlivesPredicate < I , I :: GenericArg > > ,) -> HashSet < ty :: OutlivesPredicate < I , I :: GenericArg > > { let mut collected = HashSet :: default () ; for ty :: OutlivesPredicate (arg1 , r2) in assumptions { collected . insert (ty :: OutlivesPredicate (arg1 , r2)) ; match arg1 . kind () { ty :: GenericArgKind :: Type (ty1) => { let mut components = smallvec ! [] ; push_outlives_components (cx , ty1 , & mut components) ; for c in components { match c { Component :: Region (r1) => { if ! r1 . is_bound () { collected . insert (ty :: OutlivesPredicate (r1 . into () , r2)) ; } } Component :: Param (p) => { let ty = Ty :: new_param (cx , p) ; collected . insert (ty :: OutlivesPredicate (ty . into () , r2)) ; } Component :: Placeholder (p) => { let ty = Ty :: new_placeholder (cx , p) ; collected . insert (ty :: OutlivesPredicate (ty . into () , r2)) ; } Component :: Alias (alias_ty) => { collected . insert (ty :: OutlivesPredicate (alias_ty . to_ty (cx) . into () , r2)) ; } Component :: UnresolvedInferenceVariable (_) | Component :: EscapingAlias (_) => { } } } } ty :: GenericArgKind :: Lifetime (_) => { } ty :: GenericArgKind :: Const (_) => { } } } collected }
    };
}

elaborate_outlives_assumptions!()