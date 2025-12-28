macro_rules! deps {
    () => {
        PCSTR!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl Default for PCSTR { fn default () -> Self { Self :: null () } }
    };
}

impl_109!();