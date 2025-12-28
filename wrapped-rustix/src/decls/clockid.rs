macro_rules! clockid {
    () => {
        # [cfg (not (any (windows , target_os = "espidf")))] # [cfg (any (feature = "thread" , feature = "time"))] mod clockid ;
    };
}

clockid!()