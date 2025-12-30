// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl Attributes { fn parse (& mut self , meta : syn :: meta :: ParseNestedMeta) -> syn :: parse :: Result < () > { if meta . path . is_ident ("async") { self . r#async = true ; } else if meta . path . is_ident ("crate") { self . wasm_bindgen_path = meta . value () ? . parse :: < syn :: Path > () ? ; } else if meta . path . is_ident ("unsupported") { self . unsupported = Some (meta . value () ? . parse :: < syn :: Meta > () ?) ; } else { return Err (meta . error ("unknown attribute")) ; } Ok (()) } }
};
}
