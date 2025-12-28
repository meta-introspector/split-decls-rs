macro_rules! deps {
    () => {
        Runner!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Default for Runner { fn default () -> Self { Self :: new () } }
    };
}

impl_51!();