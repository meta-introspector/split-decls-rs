macro_rules! deps {
    () => {
        PassMode!();
        ArgAbi!();
    };
}

macro_rules! classify_ret_ty {
    () => {
        deps!();
        fn classify_ret_ty < 'a , Ty , C > (arg : & mut ArgAbi < '_ , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , { if arg . is_ignore () { return ; } let mut arg_gprs_left = NUM_RET_GPRS ; classify_arg_ty (arg , & mut arg_gprs_left , MAX_RET_IN_REGS_SIZE) ; match arg . mode { super :: PassMode :: Indirect { attrs : _ , meta_attrs : _ , ref mut on_stack } => { * on_stack = false ; } _ => { } } }
    };
}

classify_ret_ty!();