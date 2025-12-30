// Generated macro for impl_352 (impl)
macro_rules! Depcrate_format_description_parse_format_itemimpl_352 {
() => {
// Module: crate::format_description::parse::format_item
// Provides: {"impl_352"}
// Dependencies: {}
impl < 'a > TryFrom < Item < 'a > > for crate :: format_description :: BorrowedFormatItem < 'a > { type Error = Error ; # [inline] fn try_from (item : Item < 'a >) -> Result < Self , Self :: Error > { match item { Item :: Literal (literal) => Ok (Self :: Literal (literal)) , Item :: Component (component) => Ok (Self :: Component (component . into ())) , Item :: Optional { value : _ , span } => Err (Error { _inner : unused (span . error ("optional items are not supported in runtime-parsed format descriptions" ,)) , public : crate :: error :: InvalidFormatDescription :: NotSupported { what : "optional item" , context : "runtime-parsed format descriptions" , index : span . start . byte as usize , } , }) , Item :: First { value : _ , span } => Err (Error { _inner : unused (span . error ("'first' items are not supported in runtime-parsed format descriptions" ,)) , public : crate :: error :: InvalidFormatDescription :: NotSupported { what : "'first' item" , context : "runtime-parsed format descriptions" , index : span . start . byte as usize , } , }) , } } }
};
}
