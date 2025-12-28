macro_rules! fn_abi_of_instance {
    () => {
        fn fn_abi_of_instance < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , (ty :: Instance < 'tcx > , & 'tcx ty :: List < Ty < 'tcx > >) > ,) -> Result < & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , & 'tcx FnAbiError < 'tcx > > { let ty :: PseudoCanonicalInput { typing_env , value : (instance , extra_args) } = query ; fn_abi_new_uncached (& LayoutCx :: new (tcx , typing_env) , fn_sig_for_fn_abi (tcx , instance , typing_env) , extra_args , Some (instance) ,) }
    };
}

fn_abi_of_instance!();