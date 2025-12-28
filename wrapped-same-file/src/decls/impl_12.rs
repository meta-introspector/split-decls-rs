macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Eq for Handle { }
    };
}

impl_12!();