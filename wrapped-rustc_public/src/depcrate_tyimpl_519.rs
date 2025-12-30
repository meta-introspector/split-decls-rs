// Generated macro for impl_519 (impl)
macro_rules! Depcrate_tyimpl_519 {
() => {
// Module: crate::ty
// Provides: {"impl_519"}
// Dependencies: {}
impl FieldDef { # [doc = " Retrieve the type of this field instantiating and normalizing it with the given arguments."] # [doc = ""] # [doc = " This will assume the type can be instantiated with these arguments."] pub fn ty_with_args (& self , args : & GenericArgs) -> Ty { with (| cx | cx . def_ty_with_args (self . def , args)) } # [doc = " Retrieve the type of this field."] pub fn ty (& self) -> Ty { with (| cx | cx . def_ty (self . def)) } }
};
}
