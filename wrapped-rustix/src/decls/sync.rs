macro_rules! sync {
    () => {
        # [cfg (not (any (target_os = "espidf" , target_os = "horizon" , target_os = "redox" , target_os = "vita" , target_os = "wasi")))] mod sync ;
    };
}

sync!()