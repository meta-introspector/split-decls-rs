macro_rules! deps {
    () => {
        Config!();
        State!();
        Lifecycle!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < C : cfg :: Config > Lifecycle < C > { const MARKED : Self = Self { state : State :: Marked , _cfg : PhantomData , } ; const REMOVING : Self = Self { state : State :: Removing , _cfg : PhantomData , } ; const PRESENT : Self = Self { state : State :: Present , _cfg : PhantomData , } ; }
    };
}

impl_98!()