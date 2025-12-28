macro_rules! deps {
    () => {
        Interner!();
        Ty!();
    };
}

macro_rules! CoercePredicate {
    () => {
        deps!();
        # [doc = " Encodes that we have to coerce *from* the `a` type to the `b` type."] # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct CoercePredicate < I : Interner > { pub a : I :: Ty , pub b : I :: Ty , }
    };
}

CoercePredicate!()