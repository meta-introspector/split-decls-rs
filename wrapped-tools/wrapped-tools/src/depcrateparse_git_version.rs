// Generated macro for parse_git_version (function)
macro_rules! Depcrateparse_git_version {
() => {
// Module: crate
// Provides: {"parse_git_version"}
// Dependencies: {}
fn parse_git_version () -> Result < (u8 , u8 , u8) > { let output = std :: process :: Command :: new (GIT_PROGRAM) . arg ("--version") . output () ? ; git_version_from_bytes (& output . stdout) }
};
}
