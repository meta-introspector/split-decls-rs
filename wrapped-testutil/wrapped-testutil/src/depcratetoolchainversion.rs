// Generated macro for ToolchainVersion (enum)
macro_rules! DepcrateToolchainVersion {
() => {
// Module: crate
// Provides: {"ToolchainVersion"}
// Dependencies: {}
# [derive (Debug)] pub enum ToolchainVersion { # [doc = " The version listed as our MSRV (ie, the `package.rust-version` key in"] # [doc = " `Cargo.toml`)."] PinnedMsrv , # [doc = " The stable version pinned in CI."] PinnedStable , # [doc = " The nightly version pinned in CI"] PinnedNightly , # [doc = " A stable version other than the one pinned in CI."] OtherStable , # [doc = " A nightly version other than the one pinned in CI."] OtherNightly , }
};
}
