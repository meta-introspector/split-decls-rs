// Generated macro for fmt_offset_minute (function)
macro_rules! Depcrate_formattingfmt_offset_minute {
() => {
// Module: crate::formatting
// Provides: {"fmt_offset_minute"}
// Dependencies: {}
# [doc = " Format the offset minute into the designated output."] # [inline] fn fmt_offset_minute (output : & mut (impl io :: Write + ? Sized) , offset : UtcOffset , modifier :: OffsetMinute { padding } : modifier :: OffsetMinute ,) -> Result < usize , io :: Error > { format_number :: < 2 > (output , offset . minutes_past_hour () . unsigned_abs () , padding) }
};
}
