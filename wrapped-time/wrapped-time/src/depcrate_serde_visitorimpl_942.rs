// Generated macro for impl_942 (impl)
macro_rules! Depcrate_serde_visitorimpl_942 {
() => {
// Module: crate::serde::visitor
// Provides: {"impl_942"}
// Dependencies: {}
impl < 'a > de :: Visitor < 'a > for Visitor < OffsetDateTime > { type Value = OffsetDateTime ; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("an `OffsetDateTime`") } # [cfg (feature = "parsing")] # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < OffsetDateTime , E > { OffsetDateTime :: parse (value , & OFFSET_DATE_TIME_FORMAT) . map_err (E :: custom) } # [inline] fn visit_seq < A : de :: SeqAccess < 'a > > (self , mut seq : A) -> Result < OffsetDateTime , A :: Error > { let year = item ! (seq , "year") ? ; let ordinal = item ! (seq , "day of year") ? ; let hour = item ! (seq , "hour") ? ; let minute = item ! (seq , "minute") ? ; let second = item ! (seq , "second") ? ; let nanosecond = item ! (seq , "nanosecond") ? ; let offset_hours = item ! (seq , "offset hours") ? ; let offset_minutes = item ! (seq , "offset minutes") ? ; let offset_seconds = item ! (seq , "offset seconds") ? ; Date :: from_ordinal_date (year , ordinal) . and_then (| date | date . with_hms_nano (hour , minute , second , nanosecond)) . and_then (| datetime | { UtcOffset :: from_hms (offset_hours , offset_minutes , offset_seconds) . map (| offset | datetime . assume_offset (offset)) }) . map_err (ComponentRange :: into_de_error) } }
};
}
