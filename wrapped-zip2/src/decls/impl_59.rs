macro_rules! deps {
    () => {
        LocalHeaderVersion!();
        ExtraFieldVersion!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl ExtraFieldVersion for LocalHeaderVersion { }
    };
}

impl_59!();