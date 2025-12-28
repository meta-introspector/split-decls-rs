macro_rules! deps {
    () => {
        Generation!();
        Config!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < C : cfg :: Config > Generation < C > { fn new (value : usize) -> Self { Self { value , _cfg : PhantomData , } } }
    };
}

impl_83!()