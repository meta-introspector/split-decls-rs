macro_rules! deps {
    () => {
        ExistentialTraitRef!();
        Interner!();
        ExistentialProjection!();
    };
}

macro_rules! ExistentialPredicate {
    () => {
        deps!();
        # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum ExistentialPredicate < I : Interner > { # [doc = " E.g., `Iterator`."] Trait (ExistentialTraitRef < I >) , # [doc = " E.g., `Iterator::Item = T`."] Projection (ExistentialProjection < I >) , # [doc = " E.g., `Send`."] AutoTrait (I :: TraitId) , }
    };
}

ExistentialPredicate!()