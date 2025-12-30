// Generated macro for schema_for (macro)
macro_rules! Depcrate_macrosschema_for {
() => {
// Module: crate::macros
// Provides: {"schema_for"}
// Dependencies: {}
# [doc = " Generates a [`Schema`](crate::Schema) for the given type using default settings."] # [doc = " The default settings currently conform to [JSON Schema 2020-12](https://json-schema.org/specification-links#2020-12), but this is liable to change in a future version of Schemars if support for other JSON Schema versions is added."] # [doc = ""] # [doc = " The type must implement [`JsonSchema`](crate::JsonSchema)."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use schemars::{schema_for, JsonSchema};"] # [doc = ""] # [doc = " #[derive(JsonSchema)]"] # [doc = " struct MyStruct {"] # [doc = "     foo: i32,"] # [doc = " }"] # [doc = ""] # [doc = " let my_schema = schema_for!(MyStruct);"] # [doc = " ```"] # [cfg (not (doc))] # [macro_export] macro_rules ! schema_for { ($ type : ty) => { $ crate :: SchemaGenerator :: default () . into_root_schema_for ::<$ type > () } ; ($ _ : expr) => { compile_error ! ("This argument to `schema_for!` is not a type - did you mean to use `schema_for_value!` instead?") } ; }
};
}
