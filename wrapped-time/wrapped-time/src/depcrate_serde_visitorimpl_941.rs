// Generated macro for impl_941 (impl)
macro_rules! Depcrate_serde_visitorimpl_941 {
() => {
// Module: crate::serde::visitor
// Provides: {"impl_941"}
// Dependencies: {}
impl < 'a > de :: Visitor < 'a > for Visitor < Duration > { type Value = Duration ; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a `Duration`") } # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < Duration , E > { let (seconds , nanoseconds) = value . split_once ('.') . ok_or_else (| | { de :: Error :: invalid_value (de :: Unexpected :: Str (value) , & "a decimal point") }) ? ; let seconds = seconds . parse () . map_err (| _ | de :: Error :: invalid_value (de :: Unexpected :: Str (seconds) , & "seconds")) ? ; let mut nanoseconds = nanoseconds . parse () . map_err (| _ | { de :: Error :: invalid_value (de :: Unexpected :: Str (nanoseconds) , & "nanoseconds") }) ? ; if seconds < 0 || (seconds == 0 && value . starts_with ("-")) { nanoseconds *= - 1 ; } Ok (Duration :: new (seconds , nanoseconds)) } # [inline] fn visit_seq < A : de :: SeqAccess < 'a > > (self , mut seq : A) -> Result < Duration , A :: Error > { let seconds = item ! (seq , "seconds") ? ; let nanoseconds = item ! (seq , "nanoseconds") ? ; Ok (Duration :: new (seconds , nanoseconds)) } }
};
}
