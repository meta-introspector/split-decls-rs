macro_rules! macro_191 {
    () => {
        cfg_not_has_atomic_u64 ! { # [path = "atomic_u64_as_mutex.rs"] mod imp ; }
    };
}

macro_191!();