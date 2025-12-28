macro_rules! deps {
    () => {
        ICause!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl ICause { # [cfg (target_os = "macos")] fn has_process (self) -> bool { true } # [cfg (not (target_os = "macos"))] fn has_process (self) -> bool { use ICause :: * ; match self { Unknown | Kernel => false , User | TKill | Queue | MesgQ | Exited | Killed | Dumped | Trapped | Stopped | Continued => true , } } }
    };
}

impl_80!()