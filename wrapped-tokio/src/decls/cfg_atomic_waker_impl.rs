macro_rules! cfg_atomic_waker_impl {
    () => {
        # [doc = " Enables internal `AtomicWaker` impl."] macro_rules ! cfg_atomic_waker_impl { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "net" , feature = "process" , feature = "rt" , feature = "signal" , feature = "time" ,))] # [cfg (not (loom))] $ item) * } }
    };
}

cfg_atomic_waker_impl!()