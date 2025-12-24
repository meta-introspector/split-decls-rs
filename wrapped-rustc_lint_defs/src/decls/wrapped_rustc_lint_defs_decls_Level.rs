use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Setting for how to handle a lint.
///
/// See: <https://doc.rust-lang.org/rustc/lints/levels.html>
#[derive(
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Debug,
    Hash,
    Encodable,
    Decodable,
    HashStable_Generic
)]
pub enum Level {
    /// The `allow` level will not issue any message.
    Allow,
    /// The `expect` level will suppress the lint message but in turn produce a message
    /// if the lint wasn't issued in the expected scope. `Expect` should not be used as
    /// an initial level for a lint.
    ///
    /// Note that this still means that the lint is enabled in this position and should
    /// be emitted, this will in turn fulfill the expectation and suppress the lint.
    ///
    /// See RFC 2383.
    ///
    /// Requires a [`LintExpectationId`] to later link a lint emission to the actual
    /// expectation. It can be ignored in most cases.
    Expect,
    /// The `warn` level will produce a warning if the lint was violated, however the
    /// compiler will continue with its execution.
    Warn,
    /// This lint level is a special case of [`Warn`], that can't be overridden. This is used
    /// to ensure that a lint can't be suppressed. This lint level can currently only be set
    /// via the console and is therefore session specific.
    ///
    /// Requires a [`LintExpectationId`] to fulfill expectations marked via the
    /// `#[expect]` attribute, that will still be suppressed due to the level.
    ForceWarn,
    /// The `deny` level will produce an error and stop further execution after the lint
    /// pass is complete.
    Deny,
    /// `Forbid` is equivalent to the `deny` level but can't be overwritten like the previous
    /// levels.
    Forbid,
}
