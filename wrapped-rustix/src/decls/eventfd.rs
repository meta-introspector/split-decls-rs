macro_rules! eventfd {
    () => {
        # [cfg (any (linux_kernel , target_os = "freebsd" , target_os = "illumos" , target_os = "espidf"))] mod eventfd ;
    };
}

eventfd!()