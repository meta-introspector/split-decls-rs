// Generated macro for StyleEdition (enum)
macro_rules! Depcrate_config_optionsStyleEdition {
() => {
// Module: crate::config::options
// Provides: {"StyleEdition"}
// Dependencies: {}
# [doc = " Defines the default values for each config according to the edition of the"] # [doc = " [Style Guide] as per [RFC 3338]. Rustfmt output may differ between Style editions."] # [doc = ""] # [doc = " [Style Guide]: https://doc.rust-lang.org/nightly/style-guide/"] # [doc = " [RFC 3338]: https://rust-lang.github.io/rfcs/3338-style-evolution.html"] # [config_type] pub enum StyleEdition { # [value = "2015"] # [doc_hint = "2015"] # [doc = " [Edition 2015]()"] Edition2015 , # [value = "2018"] # [doc_hint = "2018"] # [doc = " [Edition 2018]()"] Edition2018 , # [value = "2021"] # [doc_hint = "2021"] # [doc = " [Edition 2021]()"] Edition2021 , # [value = "2024"] # [doc_hint = "2024"] # [doc = " [Edition 2024]()."] Edition2024 , # [value = "2027"] # [doc_hint = "2027"] # [unstable_variant] # [doc = " [Edition 2027]()."] Edition2027 , }
};
}
