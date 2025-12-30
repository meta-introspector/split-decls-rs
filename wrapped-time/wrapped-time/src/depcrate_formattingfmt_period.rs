// Generated macro for fmt_period (function)
macro_rules! Depcrate_formattingfmt_period {
() => {
// Module: crate::formatting
// Provides: {"fmt_period"}
// Dependencies: {}
# [doc = " Format the period into the designated output."] # [inline] fn fmt_period (output : & mut (impl io :: Write + ? Sized) , time : Time , modifier :: Period { is_uppercase , case_sensitive : _ , } : modifier :: Period ,) -> Result < usize , io :: Error > { match (time . hour () >= 12 , is_uppercase) { (false , false) => write (output , b"am") , (false , true) => write (output , b"AM") , (true , false) => write (output , b"pm") , (true , true) => write (output , b"PM") , } }
};
}
