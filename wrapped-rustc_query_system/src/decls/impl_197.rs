macro_rules! deps {
    () => {
        DefIdCache!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < V > Default for DefIdCache < V > { fn default () -> Self { DefIdCache { local : Default :: default () , foreign : Default :: default () } } }
    };
}

impl_197!();