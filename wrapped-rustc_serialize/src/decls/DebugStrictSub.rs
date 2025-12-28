macro_rules! deps {
    () => {
        DebugStrictAdd!();
    };
}

macro_rules! DebugStrictSub {
    () => {
        deps!();
        # [doc = " See [`DebugStrictAdd`]."] pub trait DebugStrictSub { # [doc = " See [`DebugStrictAdd`]."] fn debug_strict_sub (self , other : Self) -> Self ; }
    };
}

DebugStrictSub!();