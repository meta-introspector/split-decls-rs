macro_rules! pidfd_getfd {
    () => {
        # [cfg (target_os = "linux")] mod pidfd_getfd ;
    };
}

pidfd_getfd!()