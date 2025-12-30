// Generated macro for impl_64 (impl)
macro_rules! Depcrate_struct_metaimpl_64 {
() => {
// Module: crate::struct_meta
// Provides: {"impl_64"}
// Dependencies: {}
impl Parse for ArgsForField { fn parse (input : ParseStream) -> Result < Self > { let mut name = None ; let mut unnamed = false ; for p in Punctuated :: < _ , Token ! [,] > :: parse_terminated (input) ? . into_iter () { match p { ArgForField :: Name { value , .. } => name = Some (value) , ArgForField :: Unnamed { .. } => unnamed = true , } } Ok (Self { name , unnamed }) } }
};
}
