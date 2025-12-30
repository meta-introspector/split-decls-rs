// Generated macro for impl_220 (impl)
macro_rules! Depcrate_emoji_propertiesimpl_220 {
() => {
// Module: crate::emoji_properties
// Provides: {"impl_220"}
// Dependencies: {}
impl UcdFile for EmojiProperty { fn relative_file_path () -> & 'static Path { Path :: new ("emoji/emoji-data.txt") } fn file_path < P : AsRef < Path > > (ucd_dir : P) -> PathBuf { let ucd_dir = ucd_dir . as_ref () ; let std = ucd_dir . join (Self :: relative_file_path ()) ; if std . exists () { std } else { let legacy = ucd_dir . join ("emoji-data.txt") ; if legacy . exists () { legacy } else { std } } } }
};
}
