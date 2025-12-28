macro_rules! deps {
    () => {
        UnsafeCell!();
    };
}

macro_rules! AtomicUsize {
    () => {
        deps!();
        # [doc = " `AtomicUsize` providing an additional `unsync_load` function."] pub (crate) struct AtomicUsize { inner : UnsafeCell < std :: sync :: atomic :: AtomicUsize > , }
    };
}

AtomicUsize!()