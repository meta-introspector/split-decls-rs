// Generated macro for GIT_VERSION (static)
macro_rules! DepcrateGIT_VERSION {
() => {
// Module: crate
// Provides: {"GIT_VERSION"}
// Dependencies: {}
# [doc = " The major, minor and patch level of the git version on the system."] pub static GIT_VERSION : LazyLock < (u8 , u8 , u8) > = LazyLock :: new (| | parse_git_version () . expect ("git version to be parsable")) ;
};
}
