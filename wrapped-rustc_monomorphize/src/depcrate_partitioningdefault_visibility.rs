// Generated macro for default_visibility (function)
macro_rules! Depcrate_partitioningdefault_visibility {
() => {
// Module: crate::partitioning
// Provides: {"default_visibility"}
// Dependencies: {}
fn default_visibility (tcx : TyCtxt < '_ > , id : DefId , is_generic : bool) -> Visibility { if tcx . sess . default_visibility () == SymbolVisibility :: Interposable { return Visibility :: Default ; } let export_level = if is_generic { SymbolExportLevel :: Rust } else { match tcx . reachable_non_generics (id . krate) . get (& id) { Some (SymbolExportInfo { level : SymbolExportLevel :: C , .. }) => SymbolExportLevel :: C , _ => SymbolExportLevel :: Rust , } } ; match export_level { SymbolExportLevel :: C => Visibility :: Default , SymbolExportLevel :: Rust => tcx . sess . default_visibility () . into () , } }
};
}
