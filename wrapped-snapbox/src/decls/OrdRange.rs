macro_rules! OrdRange {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] struct OrdRange { start : usize , end : usize , }
    };
}

OrdRange!()