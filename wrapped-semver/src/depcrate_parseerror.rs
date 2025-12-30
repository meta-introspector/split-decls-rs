// Generated macro for Error (struct)
macro_rules! Depcrate_parseError {
() => {
// Module: crate::parse
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error parsing a SemVer version or version requirement."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use semver::Version;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let err = Version::parse(\"1.q.r\").unwrap_err();"] # [doc = ""] # [doc = "     // \"unexpected character 'q' while parsing minor version number\""] # [doc = "     eprintln!(\"{}\", err);"] # [doc = " }"] # [doc = " ```"] pub struct Error { pub (crate) kind : ErrorKind , }
};
}
