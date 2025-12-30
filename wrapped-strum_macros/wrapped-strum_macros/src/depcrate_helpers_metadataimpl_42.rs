// Generated macro for impl_42 (impl)
macro_rules! Depcrate_helpers_metadataimpl_42 {
() => {
// Module: crate::helpers::metadata
// Provides: {"impl_42"}
// Dependencies: {}
impl VariantExt for Variant { fn get_metadata (& self) -> syn :: Result < Vec < VariantMeta > > { let result = get_metadata_inner ("strum" , & self . attrs) ? ; self . attrs . iter () . filter (| attr | attr . meta . path () . is_ident ("doc")) . try_fold (result , | mut vec , attr | { if let Meta :: NameValue (MetaNameValue { value : Expr :: Lit (ExprLit { lit : Lit :: Str (value) , .. }) , .. }) = & attr . meta { vec . push (VariantMeta :: Documentation { value : value . clone () , }) } Ok (vec) }) } }
};
}
