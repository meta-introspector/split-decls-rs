macro_rules! deps {
    () => {
        SpanRange!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl Copy for SpanRange { }
    };
}

impl_186!()