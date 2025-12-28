macro_rules! md5 {
    () => {
        # [cfg (feature = "md5")] mod md5 ;
    };
}

md5!();