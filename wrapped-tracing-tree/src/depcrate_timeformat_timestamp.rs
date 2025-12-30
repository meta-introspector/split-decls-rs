// Generated macro for format_timestamp (function)
macro_rules! Depcrate_timeformat_timestamp {
() => {
// Module: crate::time
// Provides: {"format_timestamp"}
// Dependencies: {}
fn format_timestamp (ansi : bool , elapsed : Duration , w : & mut impl Write) -> std :: fmt :: Result { let millis = elapsed . as_millis () ; let secs = elapsed . as_secs () ; let (n , unit) = if millis < 1000 { (millis as _ , "ms") } else if secs < 60 { (secs , "s ") } else { (secs / 60 , "m ") } ; let timestamp = format ! ("{n:>3}") ; write_style_timestamp (ansi , timestamp , unit , w) }
};
}
