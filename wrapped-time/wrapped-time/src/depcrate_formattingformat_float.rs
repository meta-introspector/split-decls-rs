// Generated macro for format_float (function)
macro_rules! Depcrate_formattingformat_float {
() => {
// Module: crate::formatting
// Provides: {"format_float"}
// Dependencies: {}
# [doc = " Write the floating point number to the output, returning the number of bytes written."] # [doc = ""] # [doc = " This method accepts the number of digits before and after the decimal. The value will be padded"] # [doc = " with zeroes to the left if necessary."] # [inline] pub (crate) fn format_float (output : & mut (impl io :: Write + ? Sized) , mut value : f64 , digits_before_decimal : u8 , digits_after_decimal : Option < NonZero < u8 > > ,) -> io :: Result < usize > { match digits_after_decimal { Some (digits_after_decimal) => { if digits_after_decimal . get () < 9 { let trunc_num = f64_10_pow_x (digits_after_decimal) ; value = f64 :: trunc (value * trunc_num) / trunc_num ; } let digits_after_decimal = digits_after_decimal . get () . extend () ; let width = digits_before_decimal . extend :: < usize > () + 1 + digits_after_decimal ; write ! (output , "{value:0>width$.digits_after_decimal$}") ? ; Ok (width) } None => { let value = value . trunc () as u64 ; let width = digits_before_decimal . extend () ; write ! (output , "{value:0>width$}") ? ; Ok (width) } } }
};
}
