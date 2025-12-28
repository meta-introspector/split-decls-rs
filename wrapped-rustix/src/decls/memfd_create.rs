macro_rules! memfd_create {
    () => {
        # [cfg (any (linux_kernel , target_os = "freebsd"))] mod memfd_create ;
    };
}

memfd_create!();