macro_rules! types {
    () => {
        # [cfg (not (any (target_os = "espidf" , target_os = "wasi")))] mod types ;
    };
}

types!()