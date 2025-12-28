macro_rules! deps {
    () => {
        BigEndian!();
        LittleEndian!();
    };
}

macro_rules! Order {
    () => {
        deps!();
        # [allow (missing_copy_implementations , missing_debug_implementations)] # [doc (hidden)] pub enum Order { BigEndian , LittleEndian , }
    };
}

Order!();