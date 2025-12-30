// Generated macro for generate_variadic (function)
macro_rules! Depcrate_generatorgenerate_variadic {
() => {
// Module: crate::generator
// Provides: {"generate_variadic"}
// Dependencies: {}
fn generate_variadic (variadic : bool) -> Option < TokenStream > { if variadic { Some (quote ! (variadic ,)) } else { None } }
};
}
