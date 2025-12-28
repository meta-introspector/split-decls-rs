macro_rules! deps {
    () => {
        UnsafeCell!();
    };
}

macro_rules! AtomicU16 {
    () => {
        deps!();
        # [doc = " `AtomicU16` providing an additional `unsync_load` function."] pub (crate) struct AtomicU16 { inner : UnsafeCell < std :: sync :: atomic :: AtomicU16 > , }
    };
}

AtomicU16!()