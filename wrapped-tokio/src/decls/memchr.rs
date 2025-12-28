macro_rules! memchr {
    () => {
        # [cfg (feature = "io-util")] pub (crate) mod memchr ;
    };
}

memchr!()