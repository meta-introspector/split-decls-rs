macro_rules! EarlyBinder {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct EarlyBinder < T > { pub value : T , }
    };
}

EarlyBinder!()