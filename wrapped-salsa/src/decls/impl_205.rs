macro_rules! deps {
    () => {
        Lookup!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < T > Lookup < T > for & T where T : Clone , { fn into_owned (self) -> T { Clone :: clone (self) } }
    };
}

impl_205!();