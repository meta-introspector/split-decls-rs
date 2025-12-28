macro_rules! deps {
    () => {
        SnapshotMap!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl < K , V , M , L > Default for SnapshotMap < K , V , M , L > where M : Default , L : Default , { fn default () -> Self { SnapshotMap { map : Default :: default () , undo_log : Default :: default () , _marker : PhantomData } } }
    };
}

impl_417!();