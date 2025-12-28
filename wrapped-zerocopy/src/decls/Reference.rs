macro_rules! deps {
    () => {
        Aliasing!();
        Shared!();
        Exclusive!();
    };
}

macro_rules! Reference {
    () => {
        deps!();
        # [doc = " An [`Aliasing`] invariant which is either [`Shared`] or [`Exclusive`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Given `A: Reference`, callers may assume that either `A = Shared` or `A ="] # [doc = " Exclusive`."] pub trait Reference : Aliasing + Sealed { }
    };
}

Reference!()