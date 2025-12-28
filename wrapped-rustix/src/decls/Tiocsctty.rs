macro_rules! Tiocsctty {
    () => {
        # [cfg (not (any (windows , target_os = "aix" , target_os = "horizon" , target_os = "redox" , target_os = "wasi")))] struct Tiocsctty ;
    };
}

Tiocsctty!()