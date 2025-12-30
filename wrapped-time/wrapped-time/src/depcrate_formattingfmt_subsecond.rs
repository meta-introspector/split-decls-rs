// Generated macro for fmt_subsecond (function)
macro_rules! Depcrate_formattingfmt_subsecond {
() => {
// Module: crate::formatting
// Provides: {"fmt_subsecond"}
// Dependencies: {}
# [doc = " Format the subsecond into the designated output."] # [inline] fn fmt_subsecond (output : & mut (impl io :: Write + ? Sized) , time : Time , modifier :: Subsecond { digits } : modifier :: Subsecond ,) -> Result < usize , io :: Error > { use modifier :: SubsecondDigits :: * ; let nanos = time . nanosecond () ; if digits == Nine || (digits == OneOrMore && nanos % 10 != 0) { format_number_pad_zero :: < 9 > (output , nanos) } else if digits == Eight || (digits == OneOrMore && (nanos / 10) % 10 != 0) { format_number_pad_zero :: < 8 > (output , nanos / 10) } else if digits == Seven || (digits == OneOrMore && (nanos / 100) % 10 != 0) { format_number_pad_zero :: < 7 > (output , nanos / 100) } else if digits == Six || (digits == OneOrMore && (nanos / 1_000) % 10 != 0) { format_number_pad_zero :: < 6 > (output , nanos / 1_000) } else if digits == Five || (digits == OneOrMore && (nanos / 10_000) % 10 != 0) { format_number_pad_zero :: < 5 > (output , nanos / 10_000) } else if digits == Four || (digits == OneOrMore && (nanos / 100_000) % 10 != 0) { format_number_pad_zero :: < 4 > (output , nanos / 100_000) } else if digits == Three || (digits == OneOrMore && (nanos / 1_000_000) % 10 != 0) { format_number_pad_zero :: < 3 > (output , nanos / 1_000_000) } else if digits == Two || (digits == OneOrMore && (nanos / 10_000_000) % 10 != 0) { format_number_pad_zero :: < 2 > (output , nanos / 10_000_000) } else { format_number_pad_zero :: < 1 > (output , nanos / 100_000_000) } }
};
}
