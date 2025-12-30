// Generated macro for fmt_minute (function)
macro_rules! Depcrate_formattingfmt_minute {
() => {
// Module: crate::formatting
// Provides: {"fmt_minute"}
// Dependencies: {}
# [doc = " Format the minute into the designated output."] # [inline] fn fmt_minute (output : & mut (impl io :: Write + ? Sized) , time : Time , modifier :: Minute { padding } : modifier :: Minute ,) -> Result < usize , io :: Error > { format_number :: < 2 > (output , time . minute () , padding) }
};
}
