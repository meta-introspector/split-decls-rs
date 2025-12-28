macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] impl private :: Sealed for Group { }
    };
}

impl_57!();