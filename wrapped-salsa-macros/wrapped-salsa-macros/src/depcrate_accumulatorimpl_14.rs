// Generated macro for impl_14 (impl)
macro_rules! Depcrate_accumulatorimpl_14 {
() => {
// Module: crate::accumulator
// Provides: {"impl_14"}
// Dependencies: {}
# [allow (non_snake_case)] impl StructMacro { fn try_expand (self) -> syn :: Result < TokenStream > { let ident = self . struct_item . ident . clone () ; let zalsa = self . hygiene . ident ("zalsa") ; let zalsa_struct = self . hygiene . ident ("zalsa_struct") ; let CACHE = self . hygiene . ident ("CACHE") ; let ingredient = self . hygiene . ident ("ingredient") ; let struct_item = self . struct_item ; Ok (quote ! { # struct_item salsa :: plumbing :: setup_accumulator_impl ! { Struct : # ident , unused_names : [# zalsa , # zalsa_struct , # CACHE , # ingredient ,] } }) } }
};
}
