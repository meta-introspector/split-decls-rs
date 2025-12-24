use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl VersionReq {
    /// A `VersionReq` with no constraint on the version numbers it matches.
    /// Equivalent to `VersionReq::parse("*").unwrap()`.
    ///
    /// In terms of comparators this is equivalent to `>=0.0.0`.
    ///
    /// Counterintuitively a `*` VersionReq does not match every possible
    /// version number. In particular, in order for *any* `VersionReq` to match
    /// a pre-release version, the `VersionReq` must contain at least one
    /// `Comparator` that has an explicit major, minor, and patch version
    /// identical to the pre-release being matched, and that has a nonempty
    /// pre-release component. Since `*` is not written with an explicit major,
    /// minor, and patch version, and does not contain a nonempty pre-release
    /// component, it does not match any pre-release versions.
    pub const STAR: Self = VersionReq {
        comparators: Vec::new(),
    };
    /// Create `VersionReq` by parsing from string representation.
    ///
    /// # Errors
    ///
    /// Possible reasons for the parse to fail include:
    ///
    /// - `>a.b` &mdash; unexpected characters in the partial version.
    ///
    /// - `@1.0.0` &mdash; unrecognized comparison operator.
    ///
    /// - `^1.0.0, ` &mdash; unexpected end of input.
    ///
    /// - `>=1.0 <2.0` &mdash; missing comma between comparators.
    ///
    /// - `*.*` &mdash; unsupported wildcard syntax.
    pub fn parse(text: &str) -> Result<Self, Error> {
        VersionReq::from_str(text)
    }
    /// Evaluate whether the given `Version` satisfies the version requirement
    /// described by `self`.
    pub fn matches(&self, version: &Version) -> bool {
        eval::matches_req(self, version)
    }
}
