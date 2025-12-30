// Generated macro for Container (struct)
macro_rules! Depcrate_astContainer {
() => {
// Module: crate::ast
// Provides: {"Container"}
// Dependencies: {}
pub struct Container < 'a > { pub ident : syn :: Ident , pub serde_attrs : serde_derive_internals :: attr :: Container , pub data : Data < 'a > , pub generics : syn :: Generics , pub attrs : ContainerAttrs , # [doc = " A set of type params that are used in a `rename` attribute format string, e.g. `T` and `U`"] # [doc = " in `#[schemars(rename = \"StructFor{T}And{U}\")]`. This does not include const params."] pub rename_type_params : BTreeSet < & 'a syn :: Ident > , # [doc = " A set of type params that are \"relevant\" to the impl, i.e. excluding params only used in"] # [doc = " `PhantomData` or skipped fields"] pub relevant_type_params : BTreeSet < & 'a syn :: Ident > , }
};
}
