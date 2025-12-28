macro_rules! deps {
    () => {
        WidthInfo!();
    };
}

macro_rules! str_width_cjk {
    () => {
        deps!();
        # [cfg (feature = "cjk")] # [inline] pub fn str_width_cjk (s : & str) -> usize { s . chars () . rfold ((0 , WidthInfo :: DEFAULT) , | (sum , next_info) , c | -> (usize , WidthInfo) { let (add , info) = width_in_str_cjk (c , next_info) ; (sum . wrapping_add_signed (isize :: from (add)) , info) } ,) . 0 }
    };
}

str_width_cjk!();