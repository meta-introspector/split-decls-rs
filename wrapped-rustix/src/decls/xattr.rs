macro_rules! xattr {
    () => {
        # [cfg (any (apple , linux_kernel , target_os = "hurd"))] mod xattr ;
    };
}

xattr!()