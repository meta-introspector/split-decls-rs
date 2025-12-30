// Generated macro for impl_30 (impl)
macro_rules! Depcrate_timeimpl_30 {
() => {
// Module: crate::time
// Provides: {"impl_30"}
// Dependencies: {}
# [doc = " Default do-nothing time formatter."] impl FormatTime for () { fn format_time (& self , _w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { Ok (()) } fn style_timestamp (& self , _ansi : bool , _elapsed : Duration , _w : & mut impl std :: fmt :: Write ,) -> std :: fmt :: Result { Ok (()) } }
};
}
