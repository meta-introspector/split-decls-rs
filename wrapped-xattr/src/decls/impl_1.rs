macro_rules! deps {
    () => {
        UnsupportedPlatformError!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl Error for UnsupportedPlatformError { fn description (& self) -> & str { "unsupported platform" } }
    };
}

impl_1!();