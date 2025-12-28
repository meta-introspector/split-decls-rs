macro_rules! deps {
    () => {
        FlatSet!();
        HasTop!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < T > HasTop for FlatSet < T > { const TOP : Self = Self :: Top ; }
    };
}

impl_86!()