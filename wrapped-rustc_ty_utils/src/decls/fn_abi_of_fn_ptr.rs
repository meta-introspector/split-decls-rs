macro_rules! fn_abi_of_fn_ptr {
    () => {
        fn fn_abi_of_fn_ptr < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , (ty :: PolyFnSig < 'tcx > , & 'tcx ty :: List < Ty < 'tcx > >) > ,) -> Result < & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , & 'tcx FnAbiError < 'tcx > > { let ty :: PseudoCanonicalInput { typing_env , value : (sig , extra_args) } = query ; fn_abi_new_uncached (& LayoutCx :: new (tcx , typing_env) , tcx . instantiate_bound_regions_with_erased (sig) , extra_args , None ,) }
    };
}

fn_abi_of_fn_ptr!()