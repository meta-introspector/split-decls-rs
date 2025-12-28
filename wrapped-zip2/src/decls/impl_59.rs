macro_rules! deps {
    () => {
        ExtraFieldVersion!();
        LocalHeaderVersion!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl ExtraFieldVersion for LocalHeaderVersion { }
    };
}

impl_59!()