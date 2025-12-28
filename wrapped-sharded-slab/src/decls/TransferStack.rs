macro_rules! deps {
    () => {
        DefaultConfig!();
    };
}

macro_rules! TransferStack {
    () => {
        deps!();
        pub (super) struct TransferStack < C = cfg :: DefaultConfig > { head : AtomicUsize , _cfg : PhantomData < fn (C) > , }
    };
}

TransferStack!()