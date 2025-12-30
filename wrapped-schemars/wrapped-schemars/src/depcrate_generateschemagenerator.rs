// Generated macro for SchemaGenerator (struct)
macro_rules! Depcrate_generateSchemaGenerator {
() => {
// Module: crate::generate
// Provides: {"SchemaGenerator"}
// Dependencies: {}
# [doc = " The main type used to generate JSON Schemas."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use schemars::{JsonSchema, SchemaGenerator};"] # [doc = ""] # [doc = " #[derive(JsonSchema)]"] # [doc = " struct MyStruct {"] # [doc = "     foo: i32,"] # [doc = " }"] # [doc = ""] # [doc = " let generator = SchemaGenerator::default();"] # [doc = " let schema = generator.into_root_schema_for::<MyStruct>();"] # [doc = " ```"] # [derive (Debug)] pub struct SchemaGenerator { settings : SchemaSettings , definitions : JsonMap < String , Value > , pending_schema_ids : BTreeSet < SchemaUid > , schema_id_to_name : BTreeMap < SchemaUid , CowStr > , used_schema_names : BTreeSet < CowStr > , root_schema_id_stack : Vec < SchemaUid > , }
};
}
