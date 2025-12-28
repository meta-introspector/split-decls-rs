macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! Lifecycle {
    () => {
        deps!();
        pub (crate) struct Lifecycle < C > { state : State , _cfg : PhantomData < fn (C) > , }
    };
}

Lifecycle!();