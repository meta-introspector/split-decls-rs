macro_rules! futex {
    () => {
        # [cfg (linux_kernel)] pub mod futex ;
    };
}

futex!();