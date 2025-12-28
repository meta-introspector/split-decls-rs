macro_rules! blocking {
    () => {
        # [cfg (any (feature = "fs" , feature = "io-std" , feature = "net" , all (windows , feature = "process") ,))] mod blocking ;
    };
}

blocking!();