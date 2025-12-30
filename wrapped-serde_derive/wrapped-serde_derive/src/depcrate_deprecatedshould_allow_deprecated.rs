// Generated macro for should_allow_deprecated (function)
macro_rules! Depcrate_deprecatedshould_allow_deprecated {
() => {
// Module: crate::deprecated
// Provides: {"should_allow_deprecated"}
// Dependencies: {}
# [doc = " Determine if an `#[allow(deprecated)]` should be added to the derived impl."] # [doc = ""] # [doc = " This should happen if the derive input or an enum variant it contains has"] # [doc = " one of:"] # [doc = "   - `#[deprecated]`"] # [doc = "   - `#[allow(deprecated)]`"] fn should_allow_deprecated (input : & syn :: DeriveInput) -> bool { if contains_deprecated (& input . attrs) { return true ; } if let syn :: Data :: Enum (data_enum) = & input . data { for variant in & data_enum . variants { if contains_deprecated (& variant . attrs) { return true ; } } } false }
};
}
