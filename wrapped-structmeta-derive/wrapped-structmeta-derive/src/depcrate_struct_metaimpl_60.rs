// Generated macro for impl_60 (impl)
macro_rules! Depcrate_struct_metaimpl_60 {
() => {
// Module: crate::struct_meta
// Provides: {"impl_60"}
// Dependencies: {}
impl ArgsForStruct { fn parse_from_attr (& mut self , attr : & Attribute) -> Result < () > { let args = attr . parse_args_with (Punctuated :: < ArgForStruct , Token ! [,] > :: parse_terminated) ? ; for arg in args . into_iter () { match arg { ArgForStruct :: Dump (_) => self . dump = true , ArgForStruct :: NameFilter { span , value } => { if self . name_filter . is_some () { bail ! (span , "`name_filter` cannot be specified twice") ; } self . name_filter = Some (value) ; } } } Ok (()) } fn name_filter (& self) -> NameFilter { self . name_filter . unwrap_or (NameFilter :: None) } }
};
}
