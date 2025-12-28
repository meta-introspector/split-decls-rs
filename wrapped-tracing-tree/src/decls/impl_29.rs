macro_rules! deps {
    () => {
        Uptime!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl From < std :: time :: Instant > for Uptime { fn from (epoch : std :: time :: Instant) -> Self { Uptime { epoch , higher_precision : false , } } }
    };
}

impl_29!();