macro_rules! SlabVisitor {
    () => {
        struct SlabVisitor < T > (PhantomData < T >) ;
    };
}

SlabVisitor!()