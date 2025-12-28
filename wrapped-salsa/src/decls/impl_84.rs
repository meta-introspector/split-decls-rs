macro_rules! deps {
    () => {
        DatabaseImpl!();
        Storage!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Default for DatabaseImpl { fn default () -> Self { Self { storage : Storage :: new (if tracing :: enabled ! (Level :: DEBUG) { Some (Box :: new (| event | { crate :: tracing :: debug ! ("salsa_event({:?})" , event) })) } else { None }) , } } }
    };
}

impl_84!();