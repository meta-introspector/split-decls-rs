use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// [`str`] methods producing [`SmolStr`]s.
pub trait StrExt: private::Sealed {
    /// Returns the lowercase equivalent of this string slice as a new [`SmolStr`],
    /// potentially without allocating.
    ///
    /// See [`str::to_lowercase`].
    #[must_use = "this returns a new SmolStr without modifying the original"]
    fn to_lowercase_smolstr(&self) -> SmolStr;
    /// Returns the uppercase equivalent of this string slice as a new [`SmolStr`],
    /// potentially without allocating.
    ///
    /// See [`str::to_uppercase`].
    #[must_use = "this returns a new SmolStr without modifying the original"]
    fn to_uppercase_smolstr(&self) -> SmolStr;
    /// Returns the ASCII lowercase equivalent of this string slice as a new [`SmolStr`],
    /// potentially without allocating.
    ///
    /// See [`str::to_ascii_lowercase`].
    #[must_use = "this returns a new SmolStr without modifying the original"]
    fn to_ascii_lowercase_smolstr(&self) -> SmolStr;
    /// Returns the ASCII uppercase equivalent of this string slice as a new [`SmolStr`],
    /// potentially without allocating.
    ///
    /// See [`str::to_ascii_uppercase`].
    #[must_use = "this returns a new SmolStr without modifying the original"]
    fn to_ascii_uppercase_smolstr(&self) -> SmolStr;
    /// Replaces all matches of a &str with another &str returning a new [`SmolStr`],
    /// potentially without allocating.
    ///
    /// See [`str::replace`].
    #[must_use = "this returns a new SmolStr without modifying the original"]
    fn replace_smolstr(&self, from: &str, to: &str) -> SmolStr;
    /// Replaces first N matches of a &str with another &str returning a new [`SmolStr`],
    /// potentially without allocating.
    ///
    /// See [`str::replacen`].
    #[must_use = "this returns a new SmolStr without modifying the original"]
    fn replacen_smolstr(&self, from: &str, to: &str, count: usize) -> SmolStr;
}
