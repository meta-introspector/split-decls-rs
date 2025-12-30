// Generated macro for check_default_on_tuple (function)
macro_rules! Depcrate_internals_checkcheck_default_on_tuple {
() => {
// Module: crate::internals::check
// Provides: {"check_default_on_tuple"}
// Dependencies: {}
fn check_default_on_tuple (cx : & Ctxt , cont : & Container) { if let Default :: None = cont . attrs . default () { if let Data :: Struct (Style :: Tuple , fields) = & cont . data { let mut first_default_index = None ; for (i , field) in fields . iter () . enumerate () { if field . attrs . skip_deserializing () { continue ; } if let Default :: None = field . attrs . default () { if let Some (first) = first_default_index { cx . error_spanned_by (field . ty , format ! ("field must have #[serde(default)] because previous field {} has #[serde(default)]" , first) ,) ; } continue ; } if first_default_index . is_none () { first_default_index = Some (i) ; } } } } }
};
}
