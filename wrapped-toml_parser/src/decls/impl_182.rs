macro_rules! deps {
    () => {
        EventReceiver!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl EventReceiver for () { }
    };
}

impl_182!();