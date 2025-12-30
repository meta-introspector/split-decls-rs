// Generated macro for get_version_and_channel (function)
macro_rules! Depcrate_featuresget_version_and_channel {
() => {
// Module: crate::features
// Provides: {"get_version_and_channel"}
// Dependencies: {}
fn get_version_and_channel (src_path : & Path) -> (Version , String) { let version_str = t ! (std :: fs :: read_to_string (src_path . join ("version"))) ; let version_str = version_str . trim () ; let version = t ! (std :: str :: FromStr :: from_str (version_str) . map_err (| e | format ! ("{e:?}"))) ; let channel_str = t ! (std :: fs :: read_to_string (src_path . join ("ci") . join ("channel"))) ; (version , channel_str . trim () . to_owned ()) }
};
}
