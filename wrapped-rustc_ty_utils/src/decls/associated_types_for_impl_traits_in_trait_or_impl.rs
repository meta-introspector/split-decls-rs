macro_rules! deps {
    () => {
        RPITVisitor!();
    };
}

macro_rules! associated_types_for_impl_traits_in_trait_or_impl {
    () => {
        deps!();
        fn associated_types_for_impl_traits_in_trait_or_impl < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> DefIdMap < Vec < DefId > > { let item = tcx . hir_expect_item (def_id) ; let disambiguator = & mut DisambiguatorState :: new () ; match item . kind { ItemKind :: Trait (.. , trait_item_refs) => trait_item_refs . iter () . filter_map (move | item | { if ! matches ! (tcx . def_kind (item . owner_id) , DefKind :: AssocFn) { return None ; } let fn_def_id = item . owner_id . def_id ; let Some (output) = tcx . hir_get_fn_output (fn_def_id) else { return Some ((fn_def_id . to_def_id () , vec ! [])) ; } ; let def_name = tcx . item_name (fn_def_id . to_def_id ()) ; let data = DefPathData :: AnonAssocTy (def_name) ; let mut visitor = RPITVisitor { tcx , synthetics : vec ! [] , data , disambiguator } ; visitor . visit_fn_ret_ty (output) ; let defs = visitor . synthetics . into_iter () . map (| def_id | def_id . to_def_id ()) . collect :: < Vec < _ > > () ; Some ((fn_def_id . to_def_id () , defs)) }) . collect () , ItemKind :: Impl (impl_) => { let Some (of_trait) = impl_ . of_trait else { return Default :: default () ; } ; let Some (trait_def_id) = of_trait . trait_ref . trait_def_id () else { return Default :: default () ; } ; let in_trait_def = tcx . associated_types_for_impl_traits_in_trait_or_impl (trait_def_id) ; impl_ . items . iter () . filter_map (| item | { if ! matches ! (tcx . def_kind (item . owner_id) , DefKind :: AssocFn) { return None ; } let did = item . owner_id . def_id . to_def_id () ; let item = tcx . hir_impl_item (* item) ; let ImplItemImplKind :: Trait { trait_item_def_id : Ok (trait_item_def_id) , .. } = item . impl_kind else { return Some ((did , vec ! [])) ; } ; let iter = in_trait_def [& trait_item_def_id] . iter () . map (| & id | { associated_type_for_impl_trait_in_impl (tcx , id , item , disambiguator) . to_def_id () }) ; Some ((did , iter . collect ())) }) . collect () } _ => { bug ! ("associated_types_for_impl_traits_in_trait_or_impl: {:?} should be Trait or Impl but is {:?}" , def_id , tcx . def_kind (def_id)) } } }
    };
}

associated_types_for_impl_traits_in_trait_or_impl!();