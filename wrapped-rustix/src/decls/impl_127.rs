macro_rules! deps {
    () => {
        FdSetIter!();
        FdSetElement!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'a > FdSetIter < 'a > { # [doc = " Construct a `FdSetIter` for the given set."] pub fn new (fds : & 'a [FdSetElement]) -> Self { Self { current : 0 , fds } } }
    };
}

impl_127!()