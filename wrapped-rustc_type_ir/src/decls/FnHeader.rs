macro_rules! deps {
    () => {
        Interner!();
        Abi!();
        Safety!();
    };
}

macro_rules! FnHeader {
    () => {
        deps!();
        # [derive_where (Clone , Copy , Debug , PartialEq , Hash ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] pub struct FnHeader < I : Interner > { pub c_variadic : bool , pub safety : I :: Safety , pub abi : I :: Abi , }
    };
}

FnHeader!()