macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < A > ArrayVec < A > { # [doc = " Returns the reference to the inner array of the `ArrayVec`."] # [doc = ""] # [doc = " This returns the full array, even if the `ArrayVec` length is currently"] # [doc = " less than that."] # [inline (always)] # [must_use] pub const fn as_inner (& self) -> & A { & self . data } }
    };
}

impl_23!();