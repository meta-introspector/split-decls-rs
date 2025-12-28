macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl ThreadPoolBuilder { # [doc = " Creates and returns a valid rayon thread pool builder, but does not initialize it."] pub fn new () -> Self { Self :: default () } }
    };
}

impl_28!()