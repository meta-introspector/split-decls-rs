use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Syntax tree type defined by Syn.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Node {
    /// Name of the type.
    ///
    /// This type is accessible in the Syn public API as `syn::#name`.
    pub ident: String,
    /// Features behind which this type is cfg gated.
    pub features: Features,
    /// Content of the data structure.
    #[cfg_attr(
        feature = "serde",
        serde(
            flatten,
            skip_serializing_if = "is_private",
            deserialize_with = "private_if_absent"
        )
    )]
    pub data: Data,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "is_true", default = "bool_true")
    )]
    pub exhaustive: bool,
}
