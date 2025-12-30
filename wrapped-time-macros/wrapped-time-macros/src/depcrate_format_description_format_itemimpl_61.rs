// Generated macro for impl_61 (impl)
macro_rules! Depcrate_format_description_format_itemimpl_61 {
() => {
// Module: crate::format_description::format_item
// Provides: {"impl_61"}
// Dependencies: {}
impl From < Item < '_ > > for crate :: format_description :: public :: OwnedFormatItem { fn from (item : Item < '_ >) -> Self { match item { Item :: Literal (literal) => Self :: Literal (literal . to_vec () . into_boxed_slice ()) , Item :: Component (component) => Self :: Component (component . into ()) , Item :: Optional { value , _span : _ } => Self :: Optional (Box :: new (value . into ())) , Item :: First { value , _span : _ } => { Self :: First (value . into_vec () . into_iter () . map (Into :: into) . collect ()) } } } }
};
}
