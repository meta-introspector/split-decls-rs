// Generated macro for schema_for_value (macro)
macro_rules! Depcrate_macrosschema_for_value {
() => {
// Module: crate::macros
// Provides: {"schema_for_value"}
// Dependencies: {}
# [doc = " Generates a [`Schema`](crate::Schema) for the given example value using default settings."] # [doc = " The default settings currently conform to [JSON Schema 2020-12](https://json-schema.org/specification-links#2020-12), but this is liable to change in a future version of Schemars if support for other JSON Schema versions is added."] # [doc = ""] # [doc = " The value must implement [`Serialize`](serde::Serialize). If the value also implements"] # [doc = " [`JsonSchema`](crate::JsonSchema), then prefer using the [`schema_for!(Type)`](schema_for) macro"] # [doc = " which will generally produce a more precise schema, particularly when the value contains any"] # [doc = " enums."] # [doc = ""] # [doc = " If the `Serialize` implementation of the value decides to fail, this macro will panic."] # [doc = " For a non-panicking alternative, create a [`SchemaGenerator`](crate::SchemaGenerator) and use"] # [doc = " its [`into_root_schema_for_value`](crate::SchemaGenerator::into_root_schema_for_value) method."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use schemars::schema_for_value;"] # [doc = ""] # [doc = " #[derive(serde::Serialize)]"] # [doc = " struct MyStruct {"] # [doc = "     foo: i32,"] # [doc = " }"] # [doc = ""] # [doc = " let my_schema = schema_for_value!(MyStruct { foo: 123 });"] # [doc = " ```"] # [macro_export] macro_rules ! schema_for_value { ($ value : expr) => { $ crate :: SchemaGenerator :: default () . into_root_schema_for_value (&$ value) . unwrap () } ; }
};
}
