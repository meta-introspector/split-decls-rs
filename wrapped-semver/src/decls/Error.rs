macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Error parsing a SemVer version or version requirement."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use semver::Version;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let err = Version::parse(\"1.q.r\").unwrap_err();"] # [doc = ""] # [doc = "     // \"unexpected character 'q' while parsing minor version number\""] # [doc = "     eprintln!(\"{}\", err);"] # [doc = " }"] # [doc = " ```"] pub struct Error { pub (crate) kind : ErrorKind , }
    };
}

Error!()