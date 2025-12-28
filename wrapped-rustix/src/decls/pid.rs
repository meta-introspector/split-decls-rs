macro_rules! pid {
    () => {
        # [cfg (not (any (windows , target_os = "wasi")))] # [cfg (any (feature = "process" , feature = "runtime" , feature = "termios" , feature = "thread" , all (bsd , feature = "event") , all (linux_kernel , feature = "net")))] mod pid ;
    };
}

pid!()