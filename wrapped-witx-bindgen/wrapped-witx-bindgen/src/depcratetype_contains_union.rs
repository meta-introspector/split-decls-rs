// Generated macro for type_contains_union (function)
macro_rules! Depcratetype_contains_union {
() => {
// Module: crate
// Provides: {"type_contains_union"}
// Dependencies: {}
fn type_contains_union (ty : & Type) -> bool { match ty { Type :: Variant (c) => c . cases . iter () . any (| c | c . tref . is_some ()) , Type :: List (tref) => type_contains_union (& tref . type_ ()) , Type :: Record (st) => record_contains_union (st) , _ => false , } }
};
}
