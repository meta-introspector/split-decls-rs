macro_rules! avx2 {
    () => {
        # [cfg (target_arch = "x86_64")] mod avx2 ;
    };
}

avx2!()