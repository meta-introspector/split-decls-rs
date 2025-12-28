macro_rules! deps {
    () => {
        Dispatch!();
        Subscriber!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < S > From < S > for Dispatch where S : Subscriber + Send + Sync + 'static , { # [inline] fn from (subscriber : S) -> Self { Dispatch :: new (subscriber) } }
    };
}

impl_89!();