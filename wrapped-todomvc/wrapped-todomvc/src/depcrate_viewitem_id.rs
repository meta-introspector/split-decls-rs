// Generated macro for item_id (function)
macro_rules! Depcrate_viewitem_id {
() => {
// Module: crate::view
// Provides: {"item_id"}
// Dependencies: {}
fn item_id (mut element : Element) -> Option < String > { element . parent_element () . map (| mut parent | { let mut res = None ; let parent_id = parent . dataset_get ("id") ; if ! parent_id . is_empty () { res = Some (parent_id) ; } else if let Some (mut ep) = parent . parent_element () { res = Some (ep . dataset_get ("id")) ; } res . unwrap () }) }
};
}
