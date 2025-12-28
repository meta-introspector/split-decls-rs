macro_rules! hint {
    () => {
        pub (crate) mod hint { pub (crate) use std :: hint :: spin_loop ; }
    };
}

hint!();