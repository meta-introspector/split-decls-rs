macro_rules! deps {
    () => {
        DefaultConfig!();
    };
}

macro_rules! RefCount {
    () => {
        deps!();
        # [repr (transparent)] pub (crate) struct RefCount < C = cfg :: DefaultConfig > { value : usize , _cfg : PhantomData < fn (C) > , }
    };
}

RefCount!()