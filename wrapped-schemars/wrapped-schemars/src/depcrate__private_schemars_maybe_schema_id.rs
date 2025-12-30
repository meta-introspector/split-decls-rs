// Generated macro for _schemars_maybe_schema_id (macro)
macro_rules! Depcrate__private_schemars_maybe_schema_id {
() => {
// Module: crate::_private
// Provides: {"_schemars_maybe_schema_id"}
// Dependencies: {}
# [doc = " Hack to simulate specialization:"] # [doc = " `<MaybeJsonSchemaWrapper<T>>::maybe_schema_id()` will resolve to either"] # [doc = " - The inherent method `MaybeJsonSchemaWrapper::maybe_schema_id()` if T impls `JsonSchema`"] # [doc = "     - this returns `T::schema_id()`"] # [doc = " - The trait method `NoJsonSchema::maybe_schema_id()` from the blanket impl otherwise"] # [doc = "     - this returns `core::any::type_name::<T>()``"] # [doc (hidden)] # [macro_export] macro_rules ! _schemars_maybe_schema_id { ($ ty : ty) => { { # [allow (unused_imports)] use $ crate :: _private :: { MaybeJsonSchemaWrapper , NoJsonSchema as _ } ; < MaybeJsonSchemaWrapper <$ ty >>:: maybe_schema_id () } } ; }
};
}
