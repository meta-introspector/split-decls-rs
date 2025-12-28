macro_rules! deps {
    () => {
        Abi!();
        Tys!();
        Interner!();
        Safety!();
    };
}

macro_rules! FnSig {
    () => {
        deps!();
        # [derive_where (Clone , Copy , PartialEq , Hash ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] pub struct FnSig < I : Interner > { pub inputs_and_output : I :: Tys , pub c_variadic : bool , # [type_visitable (ignore)] # [type_foldable (identity)] pub safety : I :: Safety , # [type_visitable (ignore)] # [type_foldable (identity)] pub abi : I :: Abi , }
    };
}

FnSig!();