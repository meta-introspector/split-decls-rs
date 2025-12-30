// Generated macro for fmt_hour (function)
macro_rules! Depcrate_formattingfmt_hour {
() => {
// Module: crate::formatting
// Provides: {"fmt_hour"}
// Dependencies: {}
# [doc = " Format the hour into the designated output."] # [inline] fn fmt_hour (output : & mut (impl io :: Write + ? Sized) , time : Time , modifier :: Hour { padding , is_12_hour_clock , } : modifier :: Hour ,) -> Result < usize , io :: Error > { let value = match (time . hour () , is_12_hour_clock) { (hour , false) => hour , (0 | 12 , true) => 12 , (hour , true) if hour < 12 => hour , (hour , true) => hour - 12 , } ; format_number :: < 2 > (output , value , padding) }
};
}
