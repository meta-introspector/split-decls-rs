macro_rules! create_fn_mono_item {
    () => {
        # [instrument (skip (tcx) , level = "debug" , ret)] fn create_fn_mono_item < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , source : Span ,) -> Spanned < MonoItem < 'tcx > > { let def_id = instance . def_id () ; if tcx . sess . opts . unstable_opts . profile_closures && def_id . is_local () && tcx . is_closure_like (def_id) { crate :: util :: dump_closure_profile (tcx , instance) ; } respan (source , MonoItem :: Fn (instance)) }
    };
}

create_fn_mono_item!()