macro_rules! deps {
    () => {
        TupleExt!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < T , U , V > TupleExt for (T , U , V) { type Head = T ; type Tail = V ; fn head (self) -> Self :: Head { self . 0 } fn tail (self) -> Self :: Tail { self . 2 } }
    };
}

impl_71!()