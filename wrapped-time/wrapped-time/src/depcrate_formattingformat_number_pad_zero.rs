// Generated macro for format_number_pad_zero (function)
macro_rules! Depcrate_formattingformat_number_pad_zero {
() => {
// Module: crate::formatting
// Provides: {"format_number_pad_zero"}
// Dependencies: {}
# [doc = " Format a number with the provided width and zeros as padding."] # [doc = ""] # [doc = " The sign must be written by the caller."] # [inline] pub (crate) fn format_number_pad_zero < const WIDTH : u8 > (output : & mut (impl io :: Write + ? Sized) , value : impl itoa :: Integer + DigitCount + Copy ,) -> Result < usize , io :: Error > { let mut bytes = 0 ; for _ in 0 .. (WIDTH . saturating_sub (value . num_digits ())) { bytes += write (output , b"0") ? ; } bytes += write (output , itoa :: Buffer :: new () . format (value) . as_bytes ()) ? ; Ok (bytes) }
};
}
