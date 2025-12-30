// Generated macro for impl_946 (impl)
macro_rules! Depcrate_serde_visitorimpl_946 {
() => {
// Module: crate::serde::visitor
// Provides: {"impl_946"}
// Dependencies: {}
impl < 'a > de :: Visitor < 'a > for Visitor < UtcOffset > { type Value = UtcOffset ; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a `UtcOffset`") } # [cfg (feature = "parsing")] # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < UtcOffset , E > { UtcOffset :: parse (value , & UTC_OFFSET_FORMAT) . map_err (E :: custom) } # [inline] fn visit_seq < A : de :: SeqAccess < 'a > > (self , mut seq : A) -> Result < UtcOffset , A :: Error > { let hours = item ! (seq , "offset hours") ? ; let mut minutes = 0 ; let mut seconds = 0 ; if let Ok (Some (min)) = seq . next_element () { minutes = min ; if let Ok (Some (sec)) = seq . next_element () { seconds = sec ; } } ; UtcOffset :: from_hms (hours , minutes , seconds) . map_err (ComponentRange :: into_de_error) } }
};
}
