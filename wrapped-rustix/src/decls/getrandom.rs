macro_rules! getrandom {
    () => {
        # [cfg (linux_kernel)] mod getrandom ;
    };
}

getrandom!();