macro_rules! deps {
    () => {
        HasTargetSpec!();
        FnAbi!();
    };
}

macro_rules! compute_abi_info {
    () => {
        deps!();
        pub (crate) fn compute_abi_info < 'a , Ty , C > (_cx : & C , fn_abi : & mut FnAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { if ! fn_abi . ret . is_ignore () { classify_ret_ty (& mut fn_abi . ret) ; } let mut arg_gprs_left = NUM_ARG_GPRS ; for arg in fn_abi . args . iter_mut () { if arg . is_ignore () { continue ; } classify_arg_ty (arg , & mut arg_gprs_left , MAX_ARG_IN_REGS_SIZE) ; } }
    };
}

compute_abi_info!();