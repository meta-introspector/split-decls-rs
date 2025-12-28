macro_rules! DurabilityVal {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] enum DurabilityVal { Low = 0 , Medium = 1 , High = 2 , }
    };
}

DurabilityVal!();