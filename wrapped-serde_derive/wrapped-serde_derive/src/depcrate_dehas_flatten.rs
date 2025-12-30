// Generated macro for has_flatten (function)
macro_rules! Depcrate_dehas_flatten {
() => {
// Module: crate::de
// Provides: {"has_flatten"}
// Dependencies: {}
# [doc = " True if there is any field with a `#[serde(flatten)]` attribute, other than"] # [doc = " fields which are skipped."] fn has_flatten (fields : & [Field]) -> bool { fields . iter () . any (| field | field . attrs . flatten () && ! field . attrs . skip_deserializing ()) }
};
}
