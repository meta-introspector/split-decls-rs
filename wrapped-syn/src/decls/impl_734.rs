macro_rules! deps {
    () => {
        ThreadBound!();
    };
}

macro_rules! impl_734 {
    () => {
        deps!();
        impl < T : Copy > Clone for ThreadBound < T > { fn clone (& self) -> Self { * self } }
    };
}

impl_734!()