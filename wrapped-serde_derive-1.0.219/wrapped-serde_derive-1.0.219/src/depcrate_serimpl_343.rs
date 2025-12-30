// Generated macro for impl_343 (impl)
macro_rules! Depcrate_serimpl_343 {
() => {
// Module: crate::ser
// Provides: {"impl_343"}
// Dependencies: {}
impl Parameters { fn new (cont : & Container) -> Self { let is_remote = cont . attrs . remote () . is_some () ; let self_var = if is_remote { Ident :: new ("__self" , Span :: call_site ()) } else { Ident :: new ("self" , Span :: call_site ()) } ; let this_type = this :: this_type (cont) ; let this_value = this :: this_value (cont) ; let is_packed = cont . attrs . is_packed () ; let generics = build_generics (cont) ; Parameters { self_var , this_type , this_value , generics , is_remote , is_packed , } } # [doc = " Type name to use in error messages and `&'static str` arguments to"] # [doc = " various Serializer methods."] fn type_name (& self) -> String { self . this_type . segments . last () . unwrap () . ident . to_string () } }
};
}
