// Generated macro for fmt_week_number (function)
macro_rules! Depcrate_formattingfmt_week_number {
() => {
// Module: crate::formatting
// Provides: {"fmt_week_number"}
// Dependencies: {}
# [doc = " Format the week number into the designated output."] # [inline] fn fmt_week_number (output : & mut (impl io :: Write + ? Sized) , date : Date , modifier :: WeekNumber { padding , repr } : modifier :: WeekNumber ,) -> Result < usize , io :: Error > { format_number :: < 2 > (output , match repr { modifier :: WeekNumberRepr :: Iso => date . iso_week () , modifier :: WeekNumberRepr :: Sunday => date . sunday_based_week () , modifier :: WeekNumberRepr :: Monday => date . monday_based_week () , } , padding ,) }
};
}
