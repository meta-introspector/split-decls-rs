macro_rules! openat2 {
    () => {
        # [cfg (linux_raw_dep)] mod openat2 ;
    };
}

openat2!();