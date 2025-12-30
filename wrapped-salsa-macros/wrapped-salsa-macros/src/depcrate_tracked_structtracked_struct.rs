// Generated macro for tracked_struct (function)
macro_rules! Depcrate_tracked_structtracked_struct {
() => {
// Module: crate::tracked_struct
// Provides: {"tracked_struct"}
// Dependencies: {}
# [doc = " For an entity struct `Foo` with fields `f1: T1, ..., fN: TN`, we generate..."] # [doc = ""] # [doc = " * the \"id struct\" `struct Foo(salsa::Id)`"] # [doc = " * the entity ingredient, which maps the id fields to the `Id`"] # [doc = " * for each value field, a function ingredient"] pub (crate) fn tracked_struct (args : proc_macro :: TokenStream , struct_item : syn :: ItemStruct ,) -> syn :: Result < TokenStream > { let hygiene = Hygiene :: from2 (& struct_item) ; let m = Macro { hygiene , args : syn :: parse (args) ? , struct_item , } ; m . try_macro () }
};
}
