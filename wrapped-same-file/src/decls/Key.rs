macro_rules! Key {
    () => {
        # [derive (Debug , Eq , PartialEq , Hash)] struct Key { volume : u64 , index : u64 , }
    };
}

Key!();