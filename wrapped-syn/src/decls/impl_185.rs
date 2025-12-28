macro_rules! deps {
    () => {
        SpanRange!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl Clone for SpanRange { fn clone (& self) -> Self { * self } }
    };
}

impl_185!()