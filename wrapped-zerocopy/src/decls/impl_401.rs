macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        impl < T > Split < T > { # [doc = " Produces a `Split` of `source` with `l_len`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `l_len` is no greater than `source`'s length."] # [inline (always)] unsafe fn new (source : T , l_len : usize) -> Self { Self { source , l_len } } }
    };
}

impl_401!()