macro_rules! deps {
    () => {
        Pack!();
        Generation!();
        Config!();
        Tid!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < C : cfg :: Config > Pack < C > for Generation < C > { # [doc = " Use all the remaining bits in the word for the generation counter, minus"] # [doc = " any bits reserved by the user."] const LEN : usize = (cfg :: WIDTH - C :: RESERVED_BITS) - Self :: SHIFT ; type Prev = Tid < C > ; # [inline (always)] fn from_usize (u : usize) -> Self { debug_assert ! (u <= Self :: BITS) ; Self :: new (u) } # [inline (always)] fn as_usize (& self) -> usize { self . value } }
    };
}

impl_82!()