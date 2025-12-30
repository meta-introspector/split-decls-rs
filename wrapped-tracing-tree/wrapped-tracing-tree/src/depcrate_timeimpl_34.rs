// Generated macro for impl_34 (impl)
macro_rules! Depcrate_timeimpl_34 {
() => {
// Module: crate::time
// Provides: {"impl_34"}
// Dependencies: {}
# [cfg (feature = "time")] impl FormatTime for LocalDateTime { fn format_time (& self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { let time = time :: OffsetDateTime :: now_local () . expect ("time offset cannot be determined") ; write ! (w , "{}" , time) } fn style_timestamp (& self , ansi : bool , elapsed : Duration , w : & mut impl std :: fmt :: Write ,) -> std :: fmt :: Result { style_timestamp (ansi , self . higher_precision , elapsed , w) } }
};
}
