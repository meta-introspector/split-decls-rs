// Generated macro for UcdFile (trait)
macro_rules! Depcrate_commonUcdFile {
() => {
// Module: crate::common
// Provides: {"UcdFile"}
// Dependencies: {}
# [doc = " Describes a single UCD file."] pub trait UcdFile : Clone + fmt :: Debug + Default + Eq + FromStr < Err = Error > + PartialEq { # [doc = " The file path corresponding to this file, relative to the UCD"] # [doc = " directory."] fn relative_file_path () -> & 'static Path ; # [doc = " The full file path corresponding to this file given the UCD directory"] # [doc = " path."] fn file_path < P : AsRef < Path > > (ucd_dir : P) -> PathBuf { ucd_dir . as_ref () . join (Self :: relative_file_path ()) } # [doc = " Create an iterator over each record in this UCD file."] # [doc = ""] # [doc = " The parameter should correspond to the directory containing the UCD."] fn from_dir < P : AsRef < Path > > (ucd_dir : P ,) -> Result < UcdLineParser < File , Self > , Error > { UcdLineParser :: from_path (Self :: file_path (ucd_dir)) } }
};
}
