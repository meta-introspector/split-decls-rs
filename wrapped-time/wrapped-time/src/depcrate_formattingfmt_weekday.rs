// Generated macro for fmt_weekday (function)
macro_rules! Depcrate_formattingfmt_weekday {
() => {
// Module: crate::formatting
// Provides: {"fmt_weekday"}
// Dependencies: {}
# [doc = " Format the weekday into the designated output."] # [inline] fn fmt_weekday (output : & mut (impl io :: Write + ? Sized) , date : Date , modifier :: Weekday { repr , one_indexed , case_sensitive : _ , } : modifier :: Weekday ,) -> Result < usize , io :: Error > { match repr { modifier :: WeekdayRepr :: Short => write (output , & WEEKDAY_NAMES [date . weekday () . number_days_from_monday () . extend :: < usize > ()] [.. 3] ,) , modifier :: WeekdayRepr :: Long => write (output , WEEKDAY_NAMES [date . weekday () . number_days_from_monday () . extend :: < usize > ()] ,) , modifier :: WeekdayRepr :: Sunday => format_number :: < 1 > (output , date . weekday () . number_days_from_sunday () + u8 :: from (one_indexed) , modifier :: Padding :: None ,) , modifier :: WeekdayRepr :: Monday => format_number :: < 1 > (output , date . weekday () . number_days_from_monday () + u8 :: from (one_indexed) , modifier :: Padding :: None ,) , } }
};
}
