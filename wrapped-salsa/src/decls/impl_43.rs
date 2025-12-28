macro_rules! deps {
    () => {
        IterationCount!();
        AtomicIterationCount!();
        CycleHead!();
        DatabaseKeyIndex!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl CycleHead { pub const fn new (database_key_index : DatabaseKeyIndex , iteration_count : IterationCount ,) -> Self { Self { database_key_index , iteration_count : AtomicIterationCount (AtomicU8 :: new (iteration_count . 0)) , removed : AtomicBool :: new (false) , } } }
    };
}

impl_43!();