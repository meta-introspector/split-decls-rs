macro_rules! macro_69 {
    () => {
        # [cfg (target_has_atomic = "32")] sizeof_impl ! (core :: sync :: atomic :: AtomicU32 , core :: sync :: atomic :: AtomicI32) ;
    };
}

macro_69!()