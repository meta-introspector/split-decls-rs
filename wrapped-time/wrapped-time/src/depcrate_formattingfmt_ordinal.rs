// Generated macro for fmt_ordinal (function)
macro_rules! Depcrate_formattingfmt_ordinal {
() => {
// Module: crate::formatting
// Provides: {"fmt_ordinal"}
// Dependencies: {}
# [doc = " Format the ordinal into the designated output."] # [inline] fn fmt_ordinal (output : & mut (impl io :: Write + ? Sized) , date : Date , modifier :: Ordinal { padding } : modifier :: Ordinal ,) -> Result < usize , io :: Error > { format_number :: < 3 > (output , date . ordinal () , padding) }
};
}
