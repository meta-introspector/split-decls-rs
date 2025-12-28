macro_rules! fadvise {
    () => {
        # [cfg (not (any (apple , netbsdlike , target_os = "dragonfly" , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "redox" , target_os = "solaris" , target_os = "vita" ,)))] mod fadvise ;
    };
}

fadvise!()