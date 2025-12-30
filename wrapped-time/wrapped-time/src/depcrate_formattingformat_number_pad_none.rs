// Generated macro for format_number_pad_none (function)
macro_rules! Depcrate_formattingformat_number_pad_none {
() => {
// Module: crate::formatting
// Provides: {"format_number_pad_none"}
// Dependencies: {}
# [doc = " Format a number with no padding."] # [doc = ""] # [doc = " If the sign is mandatory, the sign must be written by the caller."] # [inline] pub (crate) fn format_number_pad_none (output : & mut (impl io :: Write + ? Sized) , value : impl itoa :: Integer + Copy ,) -> Result < usize , io :: Error > { write (output , itoa :: Buffer :: new () . format (value) . as_bytes ()) }
};
}
