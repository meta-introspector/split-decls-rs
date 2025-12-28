macro_rules! madvise {
    () => {
        # [cfg (not (target_os = "redox"))] mod madvise ;
    };
}

madvise!()