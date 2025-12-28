macro_rules! userfaultfd {
    () => {
        # [cfg (linux_kernel)] mod userfaultfd ;
    };
}

userfaultfd!()