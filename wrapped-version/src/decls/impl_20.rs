macro_rules! deps {
    () => {
        OSVERSIONINFOEXW!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl OSVERSIONINFOEXW { fn new () -> Self { Self { dwOSVersionInfoSize : core :: mem :: size_of :: < Self > () as u32 , .. Default :: default () } } }
    };
}

impl_20!()