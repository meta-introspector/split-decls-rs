macro_rules! cfg_has_atomic_u64 {
    () => {
        macro_rules ! cfg_has_atomic_u64 { ($ ($ item : item) *) => { $ (# [cfg (target_has_atomic = "64")] $ item) * } }
    };
}

cfg_has_atomic_u64!();