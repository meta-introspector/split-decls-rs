// Generated macro for format_features (function)
macro_rules! Depcrate_featuresformat_features {
() => {
// Module: crate::features
// Provides: {"format_features"}
// Dependencies: {}
fn format_features < 'a > (features : & 'a Features , family : & 'a str ,) -> impl Iterator < Item = String > + 'a { features . iter () . map (move | (name , feature) | { format ! ("{:<32} {:<8} {:<12} {:<8}" , name , family , feature . level , feature . since . map_or ("None" . to_owned () , | since | since . to_string ())) }) }
};
}
