macro_rules! deps {
    () => {
        HierarchicalLayer!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl Default for HierarchicalLayer { fn default () -> Self { Self :: new (2) } }
    };
}

impl_41!()