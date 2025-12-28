macro_rules! cfg_not_taskdump {
    () => {
        macro_rules ! cfg_not_taskdump { ($ ($ item : item) *) => { $ (# [cfg (not (all (tokio_unstable , feature = "taskdump" , feature = "rt" , target_os = "linux" , any (target_arch = "aarch64" , target_arch = "x86" , target_arch = "x86_64"))))] $ item) * } ; }
    };
}

cfg_not_taskdump!()