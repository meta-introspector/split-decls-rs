macro_rules! deps {
    () => {
        ExistentialTraitRef!();
        ExistentialProjection!();
    };
}

macro_rules! ExistentialPredicate {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ExistentialPredicate { Trait (ExistentialTraitRef) , Projection (ExistentialProjection) , AutoTrait (TraitDef) , }
    };
}

ExistentialPredicate!()