// Generated macro for style_timestamp (function)
macro_rules! Depcrate_timestyle_timestamp {
() => {
// Module: crate::time
// Provides: {"style_timestamp"}
// Dependencies: {}
fn style_timestamp (ansi : bool , higher_precision : bool , elapsed : Duration , w : & mut impl Write ,) -> std :: fmt :: Result { if higher_precision { format_timestamp_with_decimals (ansi , elapsed , w) } else { format_timestamp (ansi , elapsed , w) } }
};
}
