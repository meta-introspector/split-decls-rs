macro_rules! deps {
    () => {
        FnAbi!();
        HasTargetSpec!();
    };
}

macro_rules! compute_rust_abi_info {
    () => {
        deps!();
        pub (crate) fn compute_rust_abi_info < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { if ! fn_abi . ret . is_ignore () { let has_float = match fn_abi . ret . layout . backend_repr { BackendRepr :: Scalar (s) => matches ! (s . primitive () , Primitive :: Float (_)) , BackendRepr :: ScalarPair (s1 , s2) => { matches ! (s1 . primitive () , Primitive :: Float (_)) || matches ! (s2 . primitive () , Primitive :: Float (_)) } _ => false , } ; if has_float { if cx . target_spec () . rustc_abi == Some (RustcAbi :: X86Sse2) && fn_abi . ret . layout . backend_repr . is_scalar () && fn_abi . ret . layout . size . bits () <= 128 { fn_abi . ret . cast_to (Reg { kind : RegKind :: Vector , size : fn_abi . ret . layout . size }) ; } else if fn_abi . ret . layout . size <= Primitive :: Pointer (AddressSpace :: ZERO) . size (cx) { fn_abi . ret . cast_to (Reg { kind : RegKind :: Integer , size : fn_abi . ret . layout . size }) ; } else { fn_abi . ret . make_indirect () ; } return ; } } }
    };
}

compute_rust_abi_info!()