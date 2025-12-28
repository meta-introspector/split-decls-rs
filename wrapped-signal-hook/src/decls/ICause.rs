macro_rules! ICause {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] # [repr (u8)] # [allow (dead_code)] enum ICause { Unknown = 0 , Kernel = 1 , User = 2 , TKill = 3 , Queue = 4 , MesgQ = 5 , Exited = 6 , Killed = 7 , Dumped = 8 , Trapped = 9 , Stopped = 10 , Continued = 11 , }
    };
}

ICause!()