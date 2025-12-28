macro_rules! ValidFn {
    () => {
        struct ValidFn < 'item > { db_ident : & 'item syn :: Ident , db_path : & 'item syn :: Path , }
    };
}

ValidFn!();