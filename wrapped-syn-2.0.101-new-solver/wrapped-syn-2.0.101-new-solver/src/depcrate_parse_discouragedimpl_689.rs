// Generated macro for impl_689 (impl)
macro_rules! Depcrate_parse_discouragedimpl_689 {
() => {
// Module: crate::parse::discouraged
// Provides: {"impl_689"}
// Dependencies: {}
impl < 'a > AnyDelimiter for ParseBuffer < 'a > { fn parse_any_delimiter (& self) -> Result < (Delimiter , DelimSpan , ParseBuffer) > { self . step (| cursor | { if let Some ((content , delimiter , span , rest)) = cursor . any_group () { let scope = span . close () ; let nested = crate :: parse :: advance_step_cursor (cursor , content) ; let unexpected = crate :: parse :: get_unexpected (self) ; let content = crate :: parse :: new_parse_buffer (scope , nested , unexpected) ; Ok (((delimiter , span , content) , rest)) } else { Err (cursor . error ("expected any delimiter")) } }) } }
};
}
