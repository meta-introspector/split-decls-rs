macro_rules! kill {
    () => {
        # [cfg (not (any (target_os = "espidf" , target_os = "wasi")))] mod kill ;
    };
}

kill!()