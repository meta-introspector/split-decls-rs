macro_rules! deps {
    () => {
        UnsafeCell!();
    };
}

macro_rules! AtomicU32 {
    () => {
        deps!();
        # [doc = " `AtomicU32` providing an additional `unsync_load` function."] pub (crate) struct AtomicU32 { inner : UnsafeCell < std :: sync :: atomic :: AtomicU32 > , }
    };
}

AtomicU32!()