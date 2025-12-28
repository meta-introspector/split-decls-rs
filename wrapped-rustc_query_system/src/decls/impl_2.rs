macro_rules! deps {
    () => {
        Cache!();
        Value!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < Key , Value > Default for Cache < Key , Value > { fn default () -> Self { Self { hashmap : Default :: default () } } }
    };
}

impl_2!();