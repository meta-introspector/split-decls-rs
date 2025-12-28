macro_rules! deps {
    () => {
        Attached!();
    };
}

macro_rules! macro_27 {
    () => {
        deps!();
        # [cfg (feature = "shuttle")] crate :: sync :: thread_local ! { # [doc = " The thread-local state salsa requires for a given thread"] static ATTACHED : Attached = Attached :: new () ; }
    };
}

macro_27!()