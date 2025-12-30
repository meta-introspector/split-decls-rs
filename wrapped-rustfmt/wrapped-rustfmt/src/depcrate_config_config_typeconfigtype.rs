// Generated macro for ConfigType (trait)
macro_rules! Depcrate_config_config_typeConfigType {
() => {
// Module: crate::config::config_type
// Provides: {"ConfigType"}
// Dependencies: {}
# [doc = " Trait for types that can be used in `Config`."] pub (crate) trait ConfigType : Sized { # [doc = " Returns hint text for use in `Config::print_docs()`. For enum types, this is a"] # [doc = " pipe-separated list of variants; for other types it returns `<type>`."] fn doc_hint () -> String ; # [doc = " Return `true` if the variant (i.e. value of this type) is stable."] # [doc = ""] # [doc = " By default, return true for all values. Enums annotated with `#[config_type]`"] # [doc = " are automatically implemented, based on the `#[unstable_variant]` annotation."] fn stable_variant (& self) -> bool { true } }
};
}
