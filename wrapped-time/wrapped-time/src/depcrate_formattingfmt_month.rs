// Generated macro for fmt_month (function)
macro_rules! Depcrate_formattingfmt_month {
() => {
// Module: crate::formatting
// Provides: {"fmt_month"}
// Dependencies: {}
# [doc = " Format the month into the designated output."] # [inline] fn fmt_month (output : & mut (impl io :: Write + ? Sized) , date : Date , modifier :: Month { padding , repr , case_sensitive : _ , } : modifier :: Month ,) -> Result < usize , io :: Error > { match repr { modifier :: MonthRepr :: Numerical => { format_number :: < 2 > (output , u8 :: from (date . month ()) , padding) } modifier :: MonthRepr :: Long => write (output , MONTH_NAMES [u8 :: from (date . month ()) . extend :: < usize > () - 1] ,) , modifier :: MonthRepr :: Short => write (output , & MONTH_NAMES [u8 :: from (date . month ()) . extend :: < usize > () - 1] [.. 3] ,) , } }
};
}
