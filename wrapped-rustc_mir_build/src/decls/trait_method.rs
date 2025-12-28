macro_rules! trait_method {
    () => {
        fn trait_method < 'tcx > (tcx : TyCtxt < 'tcx > , trait_def_id : DefId , method_name : Symbol , args : impl IntoIterator < Item : Into < GenericArg < 'tcx > > > ,) -> Const < 'tcx > { let item = tcx . associated_items (trait_def_id) . filter_by_name_unhygienic (method_name) . find (| item | item . is_fn ()) . expect ("trait method not found") ; let method_ty = Ty :: new_fn_def (tcx , item . def_id , args) ; Const :: zero_sized (method_ty) }
    };
}

trait_method!();