// Generated macro for impl_208 (impl)
macro_rules! Depcrate_resultimpl_208 {
() => {
// Module: crate::result
// Provides: {"impl_208"}
// Dependencies: {}
impl ZipError { # [doc = " The text used as an error when a password is required and not supplied"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use zip::result::ZipError;"] # [doc = " # let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&[])).unwrap();"] # [doc = " match archive.by_index(1) {"] # [doc = "     Err(ZipError::UnsupportedArchive(ZipError::PASSWORD_REQUIRED)) => eprintln!(\"a password is needed to unzip this file\"),"] # [doc = "     _ => (),"] # [doc = " }"] # [doc = " # ()"] # [doc = " ```"] pub const PASSWORD_REQUIRED : & 'static str = "Password required to decrypt file" ; }
};
}
