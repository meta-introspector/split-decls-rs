// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl Render for NamedType { fn render (& self , src : & mut String) { let name = self . name . as_str () ; match & self . tref { TypeRef :: Value (ty) => match & * * ty { Type :: Record (s) => render_record (src , name , s) , Type :: Handle (h) => render_handle (src , name , h) , Type :: Variant (h) => render_variant (src , name , h) , Type :: List { .. } | Type :: Pointer { .. } | Type :: ConstPointer { .. } | Type :: Builtin { .. } => render_alias (src , name , & self . tref) , } , TypeRef :: Name (_nt) => render_alias (src , name , & self . tref) , } } }
};
}
