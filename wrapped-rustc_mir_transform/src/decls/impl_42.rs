macro_rules! deps {
    () => {
        DeduceReadOnly!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl DeduceReadOnly { # [doc = " Returns a new DeduceReadOnly instance."] fn new (arg_count : usize) -> Self { Self { mutable_args : DenseBitSet :: new_empty (arg_count) } } }
    };
}

impl_42!();