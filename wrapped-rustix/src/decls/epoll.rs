macro_rules! epoll {
    () => {
        # [cfg (any (linux_kernel , target_os = "illumos" , target_os = "redox"))] pub mod epoll ;
    };
}

epoll!()