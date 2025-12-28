macro_rules! signal {
    () => {
        # [cfg (not (any (windows , target_os = "espidf" , target_os = "wasi")))] # [cfg (any (feature = "io_uring" , feature = "process" , feature = "runtime" , all (bsd , feature = "event")))] mod signal ;
    };
}

signal!();