use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Top-level content of the syntax tree description.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Definitions {
    /// The Syn version whose syntax tree is described by this data.
    pub version: Version,
    /// Syntax tree types defined by Syn.
    pub types: Vec<Node>,
    /// Token types defined by Syn (keywords as well as punctuation).
    ///
    /// The keys in the map are the Rust type name for the token. The values in
    /// the map are the printed token representation.
    ///
    /// These tokens are accessible in the Syn public API as `syn::token::#name`
    /// or alternatively `syn::Token![#repr]`.
    pub tokens: BTreeMap<String, String>,
}
