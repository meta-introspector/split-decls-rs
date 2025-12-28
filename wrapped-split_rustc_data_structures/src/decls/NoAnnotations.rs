macro_rules! NoAnnotations {
    () => {
        # [doc = " The nil annotation accumulator, which does nothing."] struct NoAnnotations < S : Idx + Ord > (PhantomData < S >) ;
    };
}

NoAnnotations!()