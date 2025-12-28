macro_rules! deps {
    () => {
        Z0!();
        B0!();
    };
}

macro_rules! Zero {
    () => {
        deps!();
        # [doc = " A **marker trait** to designate that a type is zero. Only `B0`, `U0`, and `Z0`"] # [doc = " implement this trait."] pub trait Zero : Sealed { }
    };
}

Zero!();