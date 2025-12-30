// Generated macro for get_bytepos_after_visibility (function)
macro_rules! Depcrate_itemsget_bytepos_after_visibility {
() => {
// Module: crate::items
// Provides: {"get_bytepos_after_visibility"}
// Dependencies: {}
fn get_bytepos_after_visibility (vis : & ast :: Visibility , default_span : Span) -> BytePos { match vis . kind { ast :: VisibilityKind :: Restricted { .. } => vis . span . hi () , _ => default_span . lo () , } }
};
}
