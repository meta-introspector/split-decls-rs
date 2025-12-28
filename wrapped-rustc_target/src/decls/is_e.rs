macro_rules! is_e {
    () => {
        pub (crate) fn is_e (target_features : & FxIndexSet < Symbol >) -> bool { target_features . contains (& sym :: e) }
    };
}

is_e!()