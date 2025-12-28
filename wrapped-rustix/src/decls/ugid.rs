macro_rules! ugid {
    () => {
        # [cfg (not (any (windows , target_os = "wasi")))] # [cfg (any (feature = "fs" , feature = "process" , feature = "thread" , all (linux_raw , not (feature = "use-libc-auxv") , not (feature = "use-explicitly-provided-auxv") , any (feature = "param" , feature = "runtime" , feature = "time" , target_arch = "x86" ,)) , all (linux_kernel , feature = "net")))] mod ugid ;
    };
}

ugid!()