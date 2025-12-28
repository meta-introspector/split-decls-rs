macro_rules! TupleExt {
    () => {
        pub trait TupleExt { type Head ; type Tail ; fn head (self) -> Self :: Head ; fn tail (self) -> Self :: Tail ; }
    };
}

TupleExt!()