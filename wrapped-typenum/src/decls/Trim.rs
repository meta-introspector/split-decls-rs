macro_rules! deps {
    () => {
        TrimTrailingZeros!();
        Invert!();
    };
}

macro_rules! Trim {
    () => {
        deps!();
        # [doc = " Convenience trait. Calls `Invert` -> `TrimTrailingZeros` -> `Invert`"] pub trait Trim { type Output ; fn trim (self) -> Self :: Output ; }
    };
}

Trim!();