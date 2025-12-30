// Generated macro for CheckResult (enum)
macro_rules! DepcrateCheckResult {
() => {
// Module: crate
// Provides: {"CheckResult"}
// Dependencies: {}
# [doc = " Possible results of a call to `KnownHosts::check`"] # [derive (Copy , Clone , Debug)] pub enum CheckResult { # [doc = " Hosts and keys match"] Match = raw :: LIBSSH2_KNOWNHOST_CHECK_MATCH as isize , # [doc = " Host was found, but the keys didn't match!"] Mismatch = raw :: LIBSSH2_KNOWNHOST_CHECK_MISMATCH as isize , # [doc = " No host match was found"] NotFound = raw :: LIBSSH2_KNOWNHOST_CHECK_NOTFOUND as isize , # [doc = " Something prevented the check to be made"] Failure = raw :: LIBSSH2_KNOWNHOST_CHECK_FAILURE as isize , }
};
}
