macro_rules! macro_275 {
    () => {
        # [cfg (not (target_has_atomic = "64"))] already_sync ! ([portable_atomic :: AtomicU64]) ;
    };
}

macro_275!()