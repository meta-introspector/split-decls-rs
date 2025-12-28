macro_rules! deps {
    () => {
        Array!();
    };
}

macro_rules! ArrayVecVisitor {
    () => {
        deps!();
        # [cfg (feature = "serde")] struct ArrayVecVisitor < A : Array > (PhantomData < A >) ;
    };
}

ArrayVecVisitor!();