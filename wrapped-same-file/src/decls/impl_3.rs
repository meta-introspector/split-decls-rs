macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Eq for Handle { }
    };
}

impl_3!()