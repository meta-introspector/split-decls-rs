// Generated macro for impl_238 (impl)
macro_rules! Depcrate_json_schema_impls_semver1impl_238 {
() => {
// Module: crate::json_schema_impls::semver1
// Provides: {"impl_238"}
// Dependencies: {}
impl JsonSchema for Version { fn schema_name () -> Cow < 'static , str > { "SemVer" . into () } fn schema_id () -> Cow < 'static , str > { "semver::Version" . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "string" , "pattern" : r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$" }) } }
};
}
