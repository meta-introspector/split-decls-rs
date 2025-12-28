macro_rules! deps {
    () => {
        SubscriberInitExt!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < T > SubscriberInitExt for T where T : Into < Dispatch > { }
    };
}

impl_147!()