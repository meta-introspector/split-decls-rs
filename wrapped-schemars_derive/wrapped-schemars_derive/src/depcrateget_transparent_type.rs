// Generated macro for get_transparent_type (function)
macro_rules! Depcrateget_transparent_type {
() => {
// Module: crate
// Provides: {"get_transparent_type"}
// Dependencies: {}
fn get_transparent_type < 'a > (cont : & 'a Container) -> Option < & 'a syn :: Type > { if let Some (attr :: WithAttr :: Type (ty)) = & cont . attrs . with { if cont . attrs . common . is_default () { return Some (ty) ; } } if let Some (transparent_field) = cont . transparent_field () { if cont . attrs . common . is_default () && transparent_field . attrs . is_default () { return Some (transparent_field . ty) ; } } None }
};
}
