macro_rules! deps {
    () => {
        Uptime!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Default for Uptime { fn default () -> Self { Uptime :: from (std :: time :: Instant :: now ()) } }
    };
}

impl_28!()