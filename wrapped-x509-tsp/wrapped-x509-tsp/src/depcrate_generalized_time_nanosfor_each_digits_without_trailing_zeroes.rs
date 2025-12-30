// Generated macro for for_each_digits_without_trailing_zeroes (function)
macro_rules! Depcrate_generalized_time_nanosfor_each_digits_without_trailing_zeroes {
() => {
// Module: crate::generalized_time_nanos
// Provides: {"for_each_digits_without_trailing_zeroes"}
// Dependencies: {}
fn for_each_digits_without_trailing_zeroes < F > (mut nanoseconds : u32 , mut f : F) -> Result < () > where F : FnMut (u8) -> Result < () > , { let mut idx = 100_000_000 ; while nanoseconds != 0 && idx != 0 { let cur_val = u8 :: try_from ((nanoseconds / idx) % 10) . map_err (| _ | ErrorKind :: Overflow) ? ; nanoseconds -= u32 :: from (cur_val) * idx ; idx /= 10 ; f (cur_val) ? } Ok (()) }
};
}
