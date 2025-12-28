macro_rules! macro_323 {
    () => {
        cfg_rt_multi_thread ! { mod try_lock ; pub (crate) use try_lock :: TryLock ; }
    };
}

macro_323!();