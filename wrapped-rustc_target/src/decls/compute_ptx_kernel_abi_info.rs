macro_rules! deps {
    () => {
        FnAbi!();
    };
}

macro_rules! compute_ptx_kernel_abi_info {
    () => {
        deps!();
        pub (crate) fn compute_ptx_kernel_abi_info < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { if ! fn_abi . ret . layout . is_unit () && ! fn_abi . ret . layout . is_never () { panic ! ("Kernels should not return anything other than () or !") ; } for arg in fn_abi . args . iter_mut () { if arg . is_ignore () { continue ; } classify_arg_kernel (cx , arg) ; } }
    };
}

compute_ptx_kernel_abi_info!()