macro_rules! pclmulqdq {
    () => {
        # [cfg (target_arch = "x86_64")] mod pclmulqdq ;
    };
}

pclmulqdq!()