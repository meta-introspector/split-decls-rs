// Generated macro for TypeVisitable (trait)
macro_rules! Depcrate_visitTypeVisitable {
() => {
// Module: crate::visit
// Provides: {"TypeVisitable"}
// Dependencies: {}
# [doc = " This trait is implemented for every type that can be visited,"] # [doc = " providing the skeleton of the traversal."] # [doc = ""] # [doc = " To implement this conveniently, use the derive macro located in"] # [doc = " `rustc_macros`."] pub trait TypeVisitable < I : Interner > : fmt :: Debug { # [doc = " The entry point for visiting. To visit a value `t` with a visitor `v`"] # [doc = " call: `t.visit_with(v)`."] # [doc = ""] # [doc = " For most types, this just traverses the value, calling `visit_with` on"] # [doc = " each field/element."] # [doc = ""] # [doc = " For types of interest (such as `Ty`), the implementation of this method"] # [doc = " that calls a visitor method specifically for that type (such as"] # [doc = " `V::visit_ty`). This is where control transfers from `TypeVisitable` to"] # [doc = " `TypeVisitor`."] fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result ; }
};
}
