macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! DebugConfig {
    () => {
        deps!();
        pub (crate) struct DebugConfig < C : Config > { _cfg : PhantomData < fn (C) > , }
    };
}

DebugConfig!()