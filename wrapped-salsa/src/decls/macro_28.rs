macro_rules! deps {
    () => {
        Attached!();
    };
}

macro_rules! macro_28 {
    () => {
        deps!();
        # [cfg (not (feature = "shuttle"))] crate :: sync :: thread_local ! { # [doc = " The thread-local state salsa requires for a given thread"] static ATTACHED : Attached = const { Attached :: new () } }
    };
}

macro_28!()