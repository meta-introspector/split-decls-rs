// Generated macro for impl_251 (impl)
macro_rules! Depcrate_deimpl_251 {
() => {
// Module: crate::de
// Provides: {"impl_251"}
// Dependencies: {}
impl Parameters { fn new (cont : & Container) -> Self { let local = cont . ident . clone () ; let this_type = this :: this_type (cont) ; let this_value = this :: this_value (cont) ; let borrowed = borrowed_lifetimes (cont) ; let generics = build_generics (cont , & borrowed) ; let has_getter = cont . data . has_getter () ; let is_packed = cont . attrs . is_packed () ; Parameters { local , this_type , this_value , generics , borrowed , has_getter , is_packed , } } # [doc = " Type name to use in error messages and `&'static str` arguments to"] # [doc = " various Deserializer methods."] fn type_name (& self) -> String { self . this_type . segments . last () . unwrap () . ident . to_string () } }
};
}
