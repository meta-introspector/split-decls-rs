macro_rules! deps {
    () => {
        Span!();
        SolverTraitLangItem!();
        Filter!();
        Elaboratable!();
        ClauseKind!();
        Elaborator!();
        Clause!();
        OutlivesPredicate!();
        Interner!();
        PredicatePolarity!();
        ElaborateSized!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < I : Interner , O : Elaboratable < I > > Elaborator < I , O > { # [doc = " Adds `obligations` to the stack."] fn extend_deduped (& mut self , obligations : impl IntoIterator < Item = O >) { self . stack . extend (obligations . into_iter () . filter (| o | { self . visited . insert (self . cx . anonymize_bound_vars (o . predicate () . kind ())) }) ,) ; } # [doc = " Filter to only the supertraits of trait predicates, i.e. only the predicates"] # [doc = " that have `Self` as their self type, instead of all implied predicates."] pub fn filter_only_self (mut self) -> Self { self . mode = Filter :: OnlySelf ; self } # [doc = " Start elaborating `Sized` - reqd during coherence checking, normally skipped to improve"] # [doc = " compiler performance."] pub fn elaborate_sized (mut self) -> Self { self . elaborate_sized = ElaborateSized :: Yes ; self } fn elaborate (& mut self , elaboratable : & O) { let cx = self . cx ; let Some (clause) = elaboratable . predicate () . as_clause () else { return ; } ; if self . elaborate_sized == ElaborateSized :: No && let Some (did) = clause . as_trait_clause () . map (| c | c . def_id ()) && self . cx . is_trait_lang_item (did , SolverTraitLangItem :: Sized) { return ; } let bound_clause = clause . kind () ; match bound_clause . skip_binder () { ty :: ClauseKind :: Trait (data) => { if data . polarity != ty :: PredicatePolarity :: Positive { return ; } let map_to_child_clause = | (index , (clause , span)) : (usize , (I :: Clause , I :: Span)) | { elaboratable . child_with_derived_cause (clause . instantiate_supertrait (cx , bound_clause . rebind (data . trait_ref)) , span , bound_clause . rebind (data) , index ,) } ; match self . mode { Filter :: All => self . extend_deduped (cx . explicit_implied_predicates_of (data . def_id () . into ()) . iter_identity () . enumerate () . map (map_to_child_clause) ,) , Filter :: OnlySelf => self . extend_deduped (cx . explicit_super_predicates_of (data . def_id ()) . iter_identity () . enumerate () . map (map_to_child_clause) ,) , } ; } ty :: ClauseKind :: HostEffect (data) => self . extend_deduped (cx . explicit_implied_const_bounds (data . def_id () . into ()) . iter_identity () . map (| trait_ref | { elaboratable . child (trait_ref . to_host_effect_clause (cx , data . constness) . instantiate_supertrait (cx , bound_clause . rebind (data . trait_ref)) ,) } ,) ,) , ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (ty_max , r_min)) => { if r_min . is_bound () { return ; } let mut components = smallvec ! [] ; push_outlives_components (cx , ty_max , & mut components) ; self . extend_deduped (components . into_iter () . filter_map (| component | elaborate_component_to_clause (cx , component , r_min)) . map (| clause | elaboratable . child (bound_clause . rebind (clause) . upcast (cx))) ,) ; } ty :: ClauseKind :: RegionOutlives (..) => { } ty :: ClauseKind :: WellFormed (..) => { } ty :: ClauseKind :: Projection (..) => { } ty :: ClauseKind :: ConstEvaluatable (..) => { } ty :: ClauseKind :: ConstArgHasType (..) => { } ty :: ClauseKind :: UnstableFeature (_) => { } } } }
    };
}

impl_22!()