macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! INFLATE_STATE_SIZE {
    () => {
        deps!();
        # [cfg (feature = "__internal-test")] # [doc (hidden)] pub const INFLATE_STATE_SIZE : usize = core :: mem :: size_of :: < crate :: inflate :: State > () ;
    };
}

INFLATE_STATE_SIZE!();