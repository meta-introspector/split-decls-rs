// Generated macro for fmt_offset_second (function)
macro_rules! Depcrate_formattingfmt_offset_second {
() => {
// Module: crate::formatting
// Provides: {"fmt_offset_second"}
// Dependencies: {}
# [doc = " Format the offset second into the designated output."] # [inline] fn fmt_offset_second (output : & mut (impl io :: Write + ? Sized) , offset : UtcOffset , modifier :: OffsetSecond { padding } : modifier :: OffsetSecond ,) -> Result < usize , io :: Error > { format_number :: < 2 > (output , offset . seconds_past_minute () . unsigned_abs () , padding) }
};
}
