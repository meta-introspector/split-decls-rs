macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Eq for Handle { }
    };
}

impl_20!()