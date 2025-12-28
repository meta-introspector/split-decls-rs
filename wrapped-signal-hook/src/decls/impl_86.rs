macro_rules! deps {
    () => {
        ICause!();
        Chld!();
        Cause!();
        Sent!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl From < ICause > for Cause { fn from (c : ICause) -> Cause { match c { ICause :: Kernel => Cause :: Kernel , ICause :: User => Cause :: Sent (Sent :: User) , ICause :: TKill => Cause :: Sent (Sent :: TKill) , ICause :: Queue => Cause :: Sent (Sent :: Queue) , ICause :: MesgQ => Cause :: Sent (Sent :: MesgQ) , ICause :: Exited => Cause :: Chld (Chld :: Exited) , ICause :: Killed => Cause :: Chld (Chld :: Killed) , ICause :: Dumped => Cause :: Chld (Chld :: Dumped) , ICause :: Trapped => Cause :: Chld (Chld :: Trapped) , ICause :: Stopped => Cause :: Chld (Chld :: Stopped) , ICause :: Continued => Cause :: Chld (Chld :: Continued) , _ => Cause :: Unknown , } } }
    };
}

impl_86!();