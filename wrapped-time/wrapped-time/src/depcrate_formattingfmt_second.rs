// Generated macro for fmt_second (function)
macro_rules! Depcrate_formattingfmt_second {
() => {
// Module: crate::formatting
// Provides: {"fmt_second"}
// Dependencies: {}
# [doc = " Format the second into the designated output."] # [inline] fn fmt_second (output : & mut (impl io :: Write + ? Sized) , time : Time , modifier :: Second { padding } : modifier :: Second ,) -> Result < usize , io :: Error > { format_number :: < 2 > (output , time . second () , padding) }
};
}
