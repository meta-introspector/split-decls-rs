// Generated macro for flatten_use_trees (function)
macro_rules! Depcrate_importsflatten_use_trees {
() => {
// Module: crate::imports
// Provides: {"flatten_use_trees"}
// Dependencies: {}
fn flatten_use_trees (use_trees : Vec < UseTree > , import_granularity : ImportGranularity ,) -> Vec < UseTree > { use_trees . into_iter () . flat_map (| tree | tree . flatten (import_granularity)) . map (UseTree :: nest_trailing_self) . unique () . collect () }
};
}
