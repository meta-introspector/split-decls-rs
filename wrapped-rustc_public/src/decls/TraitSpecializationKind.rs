macro_rules! TraitSpecializationKind {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum TraitSpecializationKind { None , Marker , AlwaysApplicable , }
    };
}

TraitSpecializationKind!()