macro_rules! makedev {
    () => {
        # [cfg (not (any (target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "redox" , target_os = "vita" , target_os = "wasi")))] mod makedev ;
    };
}

makedev!()