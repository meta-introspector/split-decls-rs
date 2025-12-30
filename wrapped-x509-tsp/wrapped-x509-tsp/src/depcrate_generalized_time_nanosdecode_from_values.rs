// Generated macro for decode_from_values (function)
macro_rules! Depcrate_generalized_time_nanosdecode_from_values {
() => {
// Module: crate::generalized_time_nanos
// Provides: {"decode_from_values"}
// Dependencies: {}
# [doc = " Creates a [`GeneralizedTimeNanos`] from its individual, ascii"] # [doc = " encoded components."] # [allow (clippy :: too_many_arguments , reason = "Simple helper function")] fn decode_from_values (year : (u8 , u8 , u8 , u8) , month : (u8 , u8) , day : (u8 , u8) , hour : (u8 , u8) , min : (u8 , u8) , sec : (u8 , u8) , fract : Option < & [u8] > ,) -> Result < GeneralizedTimeNanos > { let year = u16 :: from (decode_decimal (GeneralizedTimeNanos :: TAG , year . 0 , year . 1) ?) . checked_mul (100) . and_then (| y | { y . checked_add (decode_decimal (GeneralizedTimeNanos :: TAG , year . 2 , year . 3) . ok () ? . into () ,) }) . ok_or (ErrorKind :: DateTime) ? ; let month = decode_decimal (GeneralizedTimeNanos :: TAG , month . 0 , month . 1) ? ; let day = decode_decimal (GeneralizedTimeNanos :: TAG , day . 0 , day . 1) ? ; let hour = decode_decimal (GeneralizedTimeNanos :: TAG , hour . 0 , hour . 1) ? ; let minute = decode_decimal (GeneralizedTimeNanos :: TAG , min . 0 , min . 1) ? ; let second = decode_decimal (GeneralizedTimeNanos :: TAG , sec . 0 , sec . 1) ? ; let nanoseconds = if let Some (fract) = fract { decode_fractional_secs (GeneralizedTimeNanos :: TAG , fract) ? } else { 0 } ; let datetime = DateTime :: new (year , month , day , hour , minute , second) . map_err (| _ | GeneralizedTimeNanos :: TAG . value_error ()) ? ; Ok (GeneralizedTimeNanos { datetime , nanoseconds , }) }
};
}
