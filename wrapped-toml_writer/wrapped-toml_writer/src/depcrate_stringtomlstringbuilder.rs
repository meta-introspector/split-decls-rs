// Generated macro for TomlStringBuilder (struct)
macro_rules! Depcrate_stringTomlStringBuilder {
() => {
// Module: crate::string
// Provides: {"TomlStringBuilder"}
// Dependencies: {}
# [doc = " Describes how a TOML string (key or value) should be formatted."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(feature = \"alloc\")] {"] # [doc = " # use toml_writer::ToTomlValue as _;"] # [doc = " let string = \"Hello"] # [doc = " world!"] # [doc = " \";"] # [doc = " let string = toml_writer::TomlStringBuilder::new(string).as_default();"] # [doc = " let string = string.to_toml_value();"] # [doc = " assert_eq!(string, r#\"\"\"\""] # [doc = " Hello"] # [doc = " world!"] # [doc = " \"\"\"\"#);"] # [doc = " # }"] # [doc = " ```"] # [derive (Copy , Clone , Debug)] pub struct TomlStringBuilder < 's > { decoded : & 's str , metrics : ValueMetrics , }
};
}
