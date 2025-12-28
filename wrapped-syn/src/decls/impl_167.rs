macro_rules! deps {
    () => {
        TrivialDrop!();
        IntoIter!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < T > TrivialDrop for option :: IntoIter < & T > { }
    };
}

impl_167!()