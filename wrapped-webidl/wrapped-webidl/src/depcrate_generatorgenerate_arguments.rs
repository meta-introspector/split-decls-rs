// Generated macro for generate_arguments (function)
macro_rules! Depcrate_generatorgenerate_arguments {
() => {
// Module: crate::generator
// Provides: {"generate_arguments"}
// Dependencies: {}
fn generate_arguments (arguments : & [(Ident , IdlType < '_ > , Type)] , variadic : bool ,) -> Vec < TokenStream > { arguments . iter () . enumerate () . map (| (i , (name , _ , ty)) | { if variadic && i + 1 == arguments . len () { quote ! (# name : &:: js_sys :: Array) } else { quote ! (# name : # ty) } }) . collect :: < Vec < _ > > () }
};
}
