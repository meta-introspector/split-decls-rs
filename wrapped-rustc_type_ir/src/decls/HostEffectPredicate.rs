macro_rules! deps {
    () => {
        TraitRef!();
        Interner!();
        BoundConstness!();
    };
}

macro_rules! HostEffectPredicate {
    () => {
        deps!();
        # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub struct HostEffectPredicate < I : Interner > { pub trait_ref : ty :: TraitRef < I > , pub constness : BoundConstness , }
    };
}

HostEffectPredicate!()