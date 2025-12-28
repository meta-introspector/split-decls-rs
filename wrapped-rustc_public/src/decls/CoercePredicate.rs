macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! CoercePredicate {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct CoercePredicate { pub a : Ty , pub b : Ty , }
    };
}

CoercePredicate!();