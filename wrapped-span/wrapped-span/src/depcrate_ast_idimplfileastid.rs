// Generated macro for ImplFileAstId (struct)
macro_rules! Depcrate_ast_idImplFileAstId {
() => {
// Module: crate::ast_id
// Provides: {"ImplFileAstId"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] struct ImplFileAstId < 'a > { # [doc = " This can be `None` if the `Self` type is not a named type, or if it is inside a macro call."] self_ty_name : Option < & 'a str > , # [doc = " This can be `None` if this is an inherent impl, or if the trait name is inside a macro call."] trait_name : Option < & 'a str > , }
};
}
