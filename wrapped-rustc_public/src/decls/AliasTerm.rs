macro_rules! deps {
    () => {
        GenericArgs!();
    };
}

macro_rules! AliasTerm {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct AliasTerm { pub def_id : AliasDef , pub args : GenericArgs , }
    };
}

AliasTerm!();