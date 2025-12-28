macro_rules! deps {
    () => {
        TupleExt!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < T , U > TupleExt for (T , U) { type Head = T ; type Tail = U ; fn head (self) -> Self :: Head { self . 0 } fn tail (self) -> Self :: Tail { self . 1 } }
    };
}

impl_70!();