// Generated macro for OwnedFormatItem (enum)
macro_rules! Depcrate_format_description_publicOwnedFormatItem {
() => {
// Module: crate::format_description::public
// Provides: {"OwnedFormatItem"}
// Dependencies: {}
pub (crate) enum OwnedFormatItem { Literal (Box < [u8] >) , Component (Component) , Compound (Box < [Self] >) , Optional (Box < Self >) , First (Box < [Self] >) , }
};
}
