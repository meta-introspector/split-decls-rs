// Generated macro for ExtendedKind (enum)
macro_rules! Depcrate_parsing_combinator_rfc_iso8601ExtendedKind {
() => {
// Module: crate::parsing::combinator::rfc::iso8601
// Provides: {"ExtendedKind"}
// Dependencies: {}
# [doc = " What kind of format is being parsed. This is used to ensure each part of the format (date, time,"] # [doc = " offset) is the same kind."] # [derive (Debug , Clone , Copy)] pub (crate) enum ExtendedKind { # [doc = " The basic format."] Basic , # [doc = " The extended format."] Extended , # [doc = " ¯\\_(ツ)_/¯"] Unknown , }
};
}
