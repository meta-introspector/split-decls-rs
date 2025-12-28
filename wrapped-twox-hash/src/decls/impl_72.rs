macro_rules! deps {
    () => {
        OneshotWithSecretError!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl core :: error :: Error for OneshotWithSecretError { }
    };
}

impl_72!()