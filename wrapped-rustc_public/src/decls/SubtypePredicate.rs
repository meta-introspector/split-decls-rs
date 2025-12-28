macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! SubtypePredicate {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct SubtypePredicate { pub a : Ty , pub b : Ty , }
    };
}

SubtypePredicate!();