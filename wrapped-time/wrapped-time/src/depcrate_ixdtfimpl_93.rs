// Generated macro for impl_93 (impl)
macro_rules! Depcrate_ixdtfimpl_93 {
() => {
// Module: crate::ixdtf
// Provides: {"impl_93"}
// Dependencies: {}
impl From < icu_calendar :: ParseError > for ParseError { fn from (value : icu_calendar :: ParseError) -> Self { match value { icu_calendar :: ParseError :: MissingFields => Self :: MissingFields , icu_calendar :: ParseError :: Range (r) => Self :: Range (r) , icu_calendar :: ParseError :: Syntax (s) => Self :: Syntax (s) , icu_calendar :: ParseError :: UnknownCalendar => Self :: UnknownCalendar , _ => unreachable ! () , } } }
};
}
