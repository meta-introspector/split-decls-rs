macro_rules! sha1 {
    () => {
        # [cfg (feature = "sha1")] mod sha1 ;
    };
}

sha1!()