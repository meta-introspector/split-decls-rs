macro_rules! deps {
    () => {
        Config!();
        Generation!();
        Lifecycle!();
        Pack!();
        RefCount!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < C : cfg :: Config > Pack < C > for RefCount < C > { const LEN : usize = cfg :: WIDTH - (Lifecycle :: < C > :: LEN + Generation :: < C > :: LEN) ; type Prev = Lifecycle < C > ; fn from_usize (value : usize) -> Self { debug_assert ! (value <= Self :: BITS) ; Self { value , _cfg : PhantomData , } } fn as_usize (& self) -> usize { self . value } }
    };
}

impl_103!()