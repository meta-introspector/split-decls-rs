macro_rules! deps {
    () => {
        HierarchicalLayer!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Default for HierarchicalLayer { fn default () -> Self { Self :: new (2) } }
    };
}

impl_6!()