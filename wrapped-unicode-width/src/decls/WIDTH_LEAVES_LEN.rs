macro_rules! WIDTH_LEAVES_LEN {
    () => {
        # [cfg (not (feature = "cjk"))] const WIDTH_LEAVES_LEN : usize = 163 ;
    };
}

WIDTH_LEAVES_LEN!();