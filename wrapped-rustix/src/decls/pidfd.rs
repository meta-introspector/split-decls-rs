macro_rules! pidfd {
    () => {
        # [cfg (target_os = "linux")] mod pidfd ;
    };
}

pidfd!()