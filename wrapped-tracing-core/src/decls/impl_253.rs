macro_rules! deps {
    () => {
        NoSubscriber!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        impl NoSubscriber { # [doc = " Returns a new `NoSubscriber`."] # [must_use] pub const fn new () -> Self { Self (()) } }
    };
}

impl_253!();