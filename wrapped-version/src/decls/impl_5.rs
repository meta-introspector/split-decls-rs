macro_rules! impl_5 {
    () => {
        impl OSVERSIONINFOEXW { fn new () -> Self { Self { dwOSVersionInfoSize : core :: mem :: size_of :: < Self > () as u32 , .. Default :: default () } } }
    };
}

impl_5!()