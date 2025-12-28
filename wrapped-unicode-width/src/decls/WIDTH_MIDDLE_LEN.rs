macro_rules! WIDTH_MIDDLE_LEN {
    () => {
        # [cfg (not (feature = "cjk"))] const WIDTH_MIDDLE_LEN : usize = 16 ;
    };
}

WIDTH_MIDDLE_LEN!()