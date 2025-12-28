macro_rules! deps {
    () => {
        IterationCount!();
        AtomicIterationCount!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl From < IterationCount > for AtomicIterationCount { fn from (iteration_count : IterationCount) -> Self { AtomicIterationCount (iteration_count . 0 . into ()) } }
    };
}

impl_51!();