macro_rules! deps {
    () => {
        GenericArgs!();
        Interner!();
    };
}

macro_rules! OpaqueTypeKey {
    () => {
        deps!();
        # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub struct OpaqueTypeKey < I : Interner > { pub def_id : I :: LocalDefId , pub args : I :: GenericArgs , }
    };
}

OpaqueTypeKey!();