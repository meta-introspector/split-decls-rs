macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl Sealed for PathBuf { }
    };
}

impl_23!()