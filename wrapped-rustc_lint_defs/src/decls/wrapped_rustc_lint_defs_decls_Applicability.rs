use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Indicates the confidence in the correctness of a suggestion.
///
/// All suggestions are marked with an `Applicability`. Tools use the applicability of a suggestion
/// to determine whether it should be automatically applied or if the user should be consulted
/// before applying the suggestion.
#[derive(Copy, Clone, Debug, Hash, Encodable, Decodable, Serialize, Deserialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Applicability {
    /// The suggestion is definitely what the user intended, or maintains the exact meaning of the code.
    /// This suggestion should be automatically applied.
    ///
    /// In case of multiple `MachineApplicable` suggestions (whether as part of
    /// the same `multipart_suggestion` or not), all of them should be
    /// automatically applied.
    MachineApplicable,
    /// The suggestion may be what the user intended, but it is uncertain. The suggestion should
    /// result in valid Rust code if it is applied.
    MaybeIncorrect,
    /// The suggestion contains placeholders like `(...)` or `{ /* fields */ }`. The suggestion
    /// cannot be applied automatically because it will not result in valid Rust code. The user
    /// will need to fill in the placeholders.
    HasPlaceholders,
    /// The applicability of the suggestion is unknown.
    Unspecified,
}
