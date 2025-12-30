// Generated macro for impl_944 (impl)
macro_rules! Depcrate_serde_visitorimpl_944 {
() => {
// Module: crate::serde::visitor
// Provides: {"impl_944"}
// Dependencies: {}
impl < 'a > de :: Visitor < 'a > for Visitor < UtcDateTime > { type Value = UtcDateTime ; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a `PrimitiveDateTime`") } # [cfg (feature = "parsing")] # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < UtcDateTime , E > { UtcDateTime :: parse (value , & UTC_DATE_TIME_FORMAT) . map_err (E :: custom) } # [inline] fn visit_seq < A : de :: SeqAccess < 'a > > (self , mut seq : A) -> Result < UtcDateTime , A :: Error > { let year = item ! (seq , "year") ? ; let ordinal = item ! (seq , "day of year") ? ; let hour = item ! (seq , "hour") ? ; let minute = item ! (seq , "minute") ? ; let second = item ! (seq , "second") ? ; let nanosecond = item ! (seq , "nanosecond") ? ; Date :: from_ordinal_date (year , ordinal) . and_then (| date | date . with_hms_nano (hour , minute , second , nanosecond)) . map (UtcDateTime :: from_primitive) . map_err (ComponentRange :: into_de_error) } }
};
}
