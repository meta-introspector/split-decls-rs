// Generated macro for check_flatten (function)
macro_rules! Depcrate_internals_checkcheck_flatten {
() => {
// Module: crate::internals::check
// Provides: {"check_flatten"}
// Dependencies: {}
fn check_flatten (cx : & Ctxt , cont : & Container) { match & cont . data { Data :: Enum (variants) => { for variant in variants { for field in & variant . fields { check_flatten_field (cx , variant . style , field) ; } } } Data :: Struct (style , fields) => { for field in fields { check_flatten_field (cx , * style , field) ; } } } }
};
}
