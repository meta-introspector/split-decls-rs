// Generated macro for fmt_day (function)
macro_rules! Depcrate_formattingfmt_day {
() => {
// Module: crate::formatting
// Provides: {"fmt_day"}
// Dependencies: {}
# [doc = " Format the day into the designated output."] # [inline] fn fmt_day (output : & mut (impl io :: Write + ? Sized) , date : Date , modifier :: Day { padding } : modifier :: Day ,) -> Result < usize , io :: Error > { format_number :: < 2 > (output , date . day () , padding) }
};
}
