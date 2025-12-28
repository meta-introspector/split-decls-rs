macro_rules! deps {
    () => {
        CentralHeaderVersion!();
        ExtraFieldVersion!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl ExtraFieldVersion for CentralHeaderVersion { }
    };
}

impl_60!();