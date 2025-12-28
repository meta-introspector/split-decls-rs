macro_rules! macro_70 {
    () => {
        # [cfg (target_has_atomic = "64")] sizeof_impl ! (core :: sync :: atomic :: AtomicU64 , core :: sync :: atomic :: AtomicI64) ;
    };
}

macro_70!()