macro_rules! future {
    () => {
        # [cfg (any (feature = "net" , feature = "process" , feature = "signal" , feature = "sync" ,))] pub (crate) mod future { pub (crate) use crate :: sync :: AtomicWaker ; }
    };
}

future!();