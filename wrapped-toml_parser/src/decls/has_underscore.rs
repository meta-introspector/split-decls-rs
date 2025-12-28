macro_rules! has_underscore {
    () => {
        fn has_underscore (raw : & str) -> bool { raw . as_bytes () . find_slice (b'_') . is_some () }
    };
}

has_underscore!();