macro_rules! at {
    () => {
        # [cfg (not (target_os = "redox"))] mod at ;
    };
}

at!()