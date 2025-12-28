macro_rules! timerfd {
    () => {
        # [cfg (any (linux_kernel , target_os = "freebsd" , target_os = "fuchsia" , target_os = "illumos" , target_os = "netbsd"))] mod timerfd ;
    };
}

timerfd!()