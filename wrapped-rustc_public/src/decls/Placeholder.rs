macro_rules! deps {
    () => {
        UniverseIndex!();
    };
}

macro_rules! Placeholder {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct Placeholder < T > { pub universe : UniverseIndex , pub bound : T , }
    };
}

Placeholder!();