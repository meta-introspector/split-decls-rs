macro_rules! compute_hash_of_export_fn {
    () => {
        pub (crate) fn compute_hash_of_export_fn < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > ,) -> String { let def_id = instance . def_id () ; debug_assert_matches ! (tcx . def_kind (def_id) , DefKind :: Fn | DefKind :: AssocFn) ; let args = instance . args ; let sig_ty = tcx . fn_sig (def_id) . instantiate (tcx , args) ; let sig_ty = tcx . instantiate_bound_regions_with_erased (sig_ty) ; let hash = { let mut hasher = StableHasher :: new () ; sig_ty . abi_hash (tcx , & mut hasher) ; hasher . finish :: < Hash128 > () } ; hash . as_u128 () . to_string () }
    };
}

compute_hash_of_export_fn!()