macro_rules! deps {
    () => {
        AsDynDatabase!();
        Database!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < T : Database > AsDynDatabase for T { # [inline (always)] fn as_dyn_database (& self) -> & dyn Database { self } }
    };
}

impl_78!()