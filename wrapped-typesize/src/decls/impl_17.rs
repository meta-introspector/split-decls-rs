macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < Tz : chrono :: TimeZone > TypeSize for chrono :: DateTime < Tz > { }
    };
}

impl_17!()