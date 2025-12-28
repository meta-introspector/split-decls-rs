macro_rules! select {
    () => {
        # [cfg (any (bsd , linux_kernel , windows , target_os = "wasi"))] mod select ;
    };
}

select!();