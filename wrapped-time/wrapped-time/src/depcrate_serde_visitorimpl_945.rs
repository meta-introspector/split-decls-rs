// Generated macro for impl_945 (impl)
macro_rules! Depcrate_serde_visitorimpl_945 {
() => {
// Module: crate::serde::visitor
// Provides: {"impl_945"}
// Dependencies: {}
impl < 'a > de :: Visitor < 'a > for Visitor < Time > { type Value = Time ; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a `Time`") } # [cfg (feature = "parsing")] # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < Time , E > { Time :: parse (value , & TIME_FORMAT) . map_err (E :: custom) } # [inline] fn visit_seq < A : de :: SeqAccess < 'a > > (self , mut seq : A) -> Result < Time , A :: Error > { let hour = item ! (seq , "hour") ? ; let minute = item ! (seq , "minute") ? ; let second = item ! (seq , "second") ? ; let nanosecond = item ! (seq , "nanosecond") ? ; Time :: from_hms_nano (hour , minute , second , nanosecond) . map_err (ComponentRange :: into_de_error) } }
};
}
