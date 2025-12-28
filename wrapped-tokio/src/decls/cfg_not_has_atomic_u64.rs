macro_rules! cfg_not_has_atomic_u64 {
    () => {
        macro_rules ! cfg_not_has_atomic_u64 { ($ ($ item : item) *) => { $ (# [cfg (not (target_has_atomic = "64"))] $ item) * } }
    };
}

cfg_not_has_atomic_u64!()