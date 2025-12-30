// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
pub (crate) enum ErrorKind { Empty , UnexpectedEnd (Position) , UnexpectedChar (Position , char) , UnexpectedCharAfter (Position , char) , ExpectedCommaFound (Position , char) , LeadingZero (Position) , Overflow (Position) , EmptySegment (Position) , IllegalCharacter (Position) , WildcardNotTheOnlyComparator (char) , UnexpectedAfterWildcard , ExcessiveComparators , }
};
}
