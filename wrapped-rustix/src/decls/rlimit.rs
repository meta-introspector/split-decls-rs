macro_rules! rlimit {
    () => {
        # [cfg (not (any (target_os = "espidf" , target_os = "fuchsia" , target_os = "horizon" , target_os = "redox" , target_os = "vita" , target_os = "wasi")))] mod rlimit ;
    };
}

rlimit!();