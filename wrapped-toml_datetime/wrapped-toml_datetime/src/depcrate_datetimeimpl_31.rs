// Generated macro for impl_31 (impl)
macro_rules! Depcrate_datetimeimpl_31 {
() => {
// Module: crate::datetime
// Provides: {"impl_31"}
// Dependencies: {}
impl fmt :: Display for DatetimeParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (what) = self . what { write ! (f , "invalid {what}") ? ; } else { "invalid datetime" . fmt (f) ? ; } if let Some (expected) = self . expected { write ! (f , ", expected {expected}") ? ; } Ok (()) } }
};
}
