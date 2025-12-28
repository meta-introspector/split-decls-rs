macro_rules! from_buf_and_chars_size_hinted_heap {
    () => {
        # [test] fn from_buf_and_chars_size_hinted_heap () { let str = from_buf_and_chars (* b"abcdefghijklmnopqr00000" , 18 , "_0x1x2x3x4x5x6x7x8x9x10x11x12x13" . chars () ,) ; assert_eq ! (str , "abcdefghijklmnopqr_0x1x2x3x4x5x6x7x8x9x10x11x12x13") ; }
    };
}

from_buf_and_chars_size_hinted_heap!();