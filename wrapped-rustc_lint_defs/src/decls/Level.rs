macro_rules! deps {
    () => {
        LintExpectationId!();
    };
}

macro_rules! Level {
    () => {
        deps!();
        # [doc = " Setting for how to handle a lint."] # [doc = ""] # [doc = " See: <https://doc.rust-lang.org/rustc/lints/levels.html>"] # [derive (Clone , Copy , PartialEq , PartialOrd , Eq , Ord , Debug , Hash , Encodable , Decodable , HashStable_Generic)] pub enum Level { # [doc = " The `allow` level will not issue any message."] Allow , # [doc = " The `expect` level will suppress the lint message but in turn produce a message"] # [doc = " if the lint wasn't issued in the expected scope. `Expect` should not be used as"] # [doc = " an initial level for a lint."] # [doc = ""] # [doc = " Note that this still means that the lint is enabled in this position and should"] # [doc = " be emitted, this will in turn fulfill the expectation and suppress the lint."] # [doc = ""] # [doc = " See RFC 2383."] # [doc = ""] # [doc = " Requires a [`LintExpectationId`] to later link a lint emission to the actual"] # [doc = " expectation. It can be ignored in most cases."] Expect , # [doc = " The `warn` level will produce a warning if the lint was violated, however the"] # [doc = " compiler will continue with its execution."] Warn , # [doc = " This lint level is a special case of [`Warn`], that can't be overridden. This is used"] # [doc = " to ensure that a lint can't be suppressed. This lint level can currently only be set"] # [doc = " via the console and is therefore session specific."] # [doc = ""] # [doc = " Requires a [`LintExpectationId`] to fulfill expectations marked via the"] # [doc = " `#[expect]` attribute, that will still be suppressed due to the level."] ForceWarn , # [doc = " The `deny` level will produce an error and stop further execution after the lint"] # [doc = " pass is complete."] Deny , # [doc = " `Forbid` is equivalent to the `deny` level but can't be overwritten like the previous"] # [doc = " levels."] Forbid , }
    };
}

Level!();