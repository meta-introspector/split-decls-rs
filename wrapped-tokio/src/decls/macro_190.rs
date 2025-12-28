macro_rules! macro_190 {
    () => {
        cfg_has_atomic_u64 ! { # [path = "atomic_u64_native.rs"] mod imp ; }
    };
}

macro_190!();