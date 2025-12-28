macro_rules! deps {
    () => {
        FnSig!();
        Interner!();
        Ty!();
    };
}

macro_rules! impl_457 {
    () => {
        deps!();
        impl < I : Interner > FnSig < I > { pub fn inputs (self) -> I :: FnInputTys { self . inputs_and_output . inputs () } pub fn output (self) -> I :: Ty { self . inputs_and_output . output () } pub fn is_fn_trait_compatible (self) -> bool { let FnSig { safety , abi , c_variadic , .. } = self ; ! c_variadic && safety . is_safe () && abi . is_rust () } }
    };
}

impl_457!();