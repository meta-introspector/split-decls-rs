// Generated macro for impl_38 (impl)
macro_rules! Depcrate_timeimpl_38 {
() => {
// Module: crate::time
// Provides: {"impl_38"}
// Dependencies: {}
impl FormatTime for Uptime { fn format_time (& self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { let e = self . epoch . elapsed () ; write ! (w , "{:4}.{:06}s" , e . as_secs () , e . subsec_micros ()) } fn style_timestamp (& self , ansi : bool , elapsed : Duration , w : & mut impl std :: fmt :: Write ,) -> std :: fmt :: Result { style_timestamp (ansi , self . higher_precision , elapsed , w) } }
};
}
