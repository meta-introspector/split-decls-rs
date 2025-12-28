macro_rules! deps {
    () => {
        AsMap!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl AsMap for Event < '_ > { }
    };
}

impl_2!()