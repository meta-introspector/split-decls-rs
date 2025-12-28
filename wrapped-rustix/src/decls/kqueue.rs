macro_rules! kqueue {
    () => {
        # [cfg (bsd)] pub mod kqueue ;
    };
}

kqueue!()