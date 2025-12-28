macro_rules! deps {
    () => {
        AliasTerm!();
        Term!();
        Interner!();
    };
}

macro_rules! NormalizesTo {
    () => {
        deps!();
        # [doc = " Used by the new solver to normalize an alias. This always expects the `term` to"] # [doc = " be an unconstrained inference variable which is used as the output."] # [derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct NormalizesTo < I : Interner > { pub alias : AliasTerm < I > , pub term : I :: Term , }
    };
}

NormalizesTo!()