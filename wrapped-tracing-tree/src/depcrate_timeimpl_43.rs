// Generated macro for impl_43 (impl)
macro_rules! Depcrate_timeimpl_43 {
() => {
// Module: crate::time
// Provides: {"impl_43"}
// Dependencies: {}
impl < F > FormatTime for & F where F : FormatTime , { fn format_time (& self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { F :: format_time (self , w) } fn style_timestamp (& self , ansi : bool , duration : Duration , w : & mut impl std :: fmt :: Write ,) -> std :: fmt :: Result { F :: style_timestamp (self , ansi , duration , w) } }
};
}
