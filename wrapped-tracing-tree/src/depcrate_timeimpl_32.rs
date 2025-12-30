// Generated macro for impl_32 (impl)
macro_rules! Depcrate_timeimpl_32 {
() => {
// Module: crate::time
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (feature = "time")] impl FormatTime for UtcDateTime { fn format_time (& self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { let time = time :: OffsetDateTime :: now_utc () ; write ! (w , "{} {}" , time . date () , time . time ()) } fn style_timestamp (& self , ansi : bool , elapsed : Duration , w : & mut impl std :: fmt :: Write ,) -> std :: fmt :: Result { style_timestamp (ansi , self . higher_precision , elapsed , w) } }
};
}
