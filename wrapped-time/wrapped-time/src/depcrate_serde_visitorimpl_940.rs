// Generated macro for impl_940 (impl)
macro_rules! Depcrate_serde_visitorimpl_940 {
() => {
// Module: crate::serde::visitor
// Provides: {"impl_940"}
// Dependencies: {}
impl < 'a > de :: Visitor < 'a > for Visitor < Date > { type Value = Date ; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a `Date`") } # [cfg (feature = "parsing")] # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < Date , E > { Date :: parse (value , & DATE_FORMAT) . map_err (E :: custom) } # [inline] fn visit_seq < A : de :: SeqAccess < 'a > > (self , mut seq : A) -> Result < Date , A :: Error > { let year = item ! (seq , "year") ? ; let ordinal = item ! (seq , "day of year") ? ; Date :: from_ordinal_date (year , ordinal) . map_err (ComponentRange :: into_de_error) } }
};
}
