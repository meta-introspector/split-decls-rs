macro_rules! deps {
    () => {
        Z0!();
        B0!();
    };
}

macro_rules! NonZero {
    () => {
        deps!();
        # [doc = " A **marker trait** to designate that a type is not zero. All number types in this"] # [doc = " crate implement `NonZero` except `B0`, `U0`, and `Z0`."] pub trait NonZero : Sealed { }
    };
}

NonZero!();