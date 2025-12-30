// Generated macro for collect_unstable_book_section_file_names (function)
macro_rules! Depcrate_unstable_bookcollect_unstable_book_section_file_names {
() => {
// Module: crate::unstable_book
// Provides: {"collect_unstable_book_section_file_names"}
// Dependencies: {}
pub fn collect_unstable_book_section_file_names (dir : & Path) -> BTreeSet < String > { fs :: read_dir (dir) . expect ("could not read directory") . map (| entry | entry . expect ("could not read directory entry")) . filter (dir_entry_is_file) . map (| entry | entry . path ()) . filter (| path | path . extension () . map (| e | e . to_str () . unwrap ()) == Some ("md")) . map (| path | path . file_stem () . unwrap () . to_str () . unwrap () . into ()) . collect () }
};
}
