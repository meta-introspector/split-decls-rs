macro_rules! deps {
    () => {
        GenericArgs!();
    };
}

macro_rules! AliasTy {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct AliasTy { pub def_id : AliasDef , pub args : GenericArgs , }
    };
}

AliasTy!();