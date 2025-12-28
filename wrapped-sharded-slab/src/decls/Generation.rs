macro_rules! deps {
    () => {
        DefaultConfig!();
    };
}

macro_rules! Generation {
    () => {
        deps!();
        # [repr (transparent)] pub (crate) struct Generation < C = cfg :: DefaultConfig > { value : usize , _cfg : PhantomData < fn (C) > , }
    };
}

Generation!()