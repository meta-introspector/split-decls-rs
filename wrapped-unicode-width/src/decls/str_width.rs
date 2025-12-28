macro_rules! deps {
    () => {
        WidthInfo!();
    };
}

macro_rules! str_width {
    () => {
        deps!();
        # [inline] pub fn str_width (s : & str) -> usize { s . chars () . rfold ((0 , WidthInfo :: DEFAULT) , | (sum , next_info) , c | -> (usize , WidthInfo) { let (add , info) = width_in_str (c , next_info) ; (sum . wrapping_add_signed (isize :: from (add)) , info) } ,) . 0 }
    };
}

str_width!();