macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < K , V > Default for IndexMap < K , V > { fn default () -> Self { Self { index_map : FxIndexMap :: default () } } }
    };
}

impl_61!();