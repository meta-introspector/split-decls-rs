macro_rules! sched {
    () => {
        # [cfg (any (freebsdlike , linux_kernel , target_os = "fuchsia"))] mod sched ;
    };
}

sched!();