macro_rules! deps {
    () => {
        SubscriberExt!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < S : Subscriber > SubscriberExt for S { }
    };
}

impl_142!()