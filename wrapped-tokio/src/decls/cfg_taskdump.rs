macro_rules! cfg_taskdump {
    () => {
        macro_rules ! cfg_taskdump { ($ ($ item : item) *) => { $ (# [cfg (all (tokio_unstable , feature = "taskdump" , feature = "rt" , target_os = "linux" , any (target_arch = "aarch64" , target_arch = "x86" , target_arch = "x86_64")))] $ item) * } ; }
    };
}

cfg_taskdump!();