// Generated macro for format_number (function)
macro_rules! Depcrate_formattingformat_number {
() => {
// Module: crate::formatting
// Provides: {"format_number"}
// Dependencies: {}
# [doc = " Format a number with the provided padding and width."] # [doc = ""] # [doc = " The sign must be written by the caller."] # [inline] pub (crate) fn format_number < const WIDTH : u8 > (output : & mut (impl io :: Write + ? Sized) , value : impl itoa :: Integer + DigitCount + Copy , padding : modifier :: Padding ,) -> Result < usize , io :: Error > { match padding { modifier :: Padding :: Space => format_number_pad_space :: < WIDTH > (output , value) , modifier :: Padding :: Zero => format_number_pad_zero :: < WIDTH > (output , value) , modifier :: Padding :: None => format_number_pad_none (output , value) , } }
};
}
