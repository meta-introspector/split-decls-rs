macro_rules! deps {
    () => {
        Array!();
    };
}

macro_rules! TinyVecVisitor {
    () => {
        deps!();
        # [cfg (feature = "serde")] # [cfg_attr (docs_rs , doc (cfg (feature = "alloc")))] struct TinyVecVisitor < A : Array > (PhantomData < A >) ;
    };
}

TinyVecVisitor!()