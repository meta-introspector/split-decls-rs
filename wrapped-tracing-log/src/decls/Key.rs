macro_rules! Key {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Hash)] struct Key { target_address : usize , level_and_length : usize , }
    };
}

Key!();