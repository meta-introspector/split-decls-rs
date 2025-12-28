macro_rules! deps {
    () => {
        MaybeTransitiveLiveLocals!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < 'a > MaybeTransitiveLiveLocals < 'a > { # [doc = " The `always_alive` set is the set of locals to which all stores should unconditionally be"] # [doc = " considered live."] # [doc = ""] # [doc = " This should include at least all locals that are ever borrowed."] pub fn new (always_live : & 'a DenseBitSet < Local >) -> Self { MaybeTransitiveLiveLocals { always_live } } }
    };
}

impl_161!()