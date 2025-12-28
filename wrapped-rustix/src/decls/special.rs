macro_rules! special {
    () => {
        # [cfg (not (any (target_os = "espidf" , target_os = "redox")))] mod special ;
    };
}

special!()