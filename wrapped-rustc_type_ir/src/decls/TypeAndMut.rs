macro_rules! deps {
    () => {
        Ty!();
        Interner!();
    };
}

macro_rules! TypeAndMut {
    () => {
        deps!();
        # [derive_where (Clone , Copy , PartialEq , Hash , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] pub struct TypeAndMut < I : Interner > { pub ty : I :: Ty , pub mutbl : Mutability , }
    };
}

TypeAndMut!()