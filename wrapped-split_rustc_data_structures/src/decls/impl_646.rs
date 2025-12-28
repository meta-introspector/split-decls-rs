macro_rules! deps {
    () => {
        UnordSet!();
    };
}

macro_rules! impl_646 {
    () => {
        deps!();
        impl < V : Eq + Hash > Default for UnordSet < V > { # [inline] fn default () -> Self { Self { inner : FxHashSet :: default () } } }
    };
}

impl_646!();