macro_rules! deps {
    () => {
        InflateConfig!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl Default for InflateConfig { fn default () -> Self { Self { window_bits : DEF_WBITS , } } }
    };
}

impl_247!();