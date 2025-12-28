macro_rules! ParamLike {
    () => {
        pub trait ParamLike : Copy + Debug + Hash + Eq { fn index (self) -> u32 ; }
    };
}

ParamLike!();