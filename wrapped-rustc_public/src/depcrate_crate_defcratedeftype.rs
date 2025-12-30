// Generated macro for CrateDefType (trait)
macro_rules! Depcrate_crate_defCrateDefType {
() => {
// Module: crate::crate_def
// Provides: {"CrateDefType"}
// Dependencies: {}
# [doc = " A trait that can be used to retrieve a definition's type."] # [doc = ""] # [doc = " Note that not every CrateDef has a type `Ty`. They should not implement this trait."] pub trait CrateDefType : CrateDef { # [doc = " Returns the type of this crate item."] fn ty (& self) -> Ty { with (| cx | cx . def_ty (self . def_id ())) } # [doc = " Retrieve the type of this definition by instantiating and normalizing it with `args`."] # [doc = ""] # [doc = " This will panic if instantiation fails."] fn ty_with_args (& self , args : & GenericArgs) -> Ty { with (| cx | cx . def_ty_with_args (self . def_id () , args)) } }
};
}
