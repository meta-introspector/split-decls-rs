// Generated macro for impl_115 (impl)
macro_rules! Depcrate_publishimpl_115 {
() => {
// Module: crate::publish
// Provides: {"impl_115"}
// Dependencies: {}
impl flags :: PublishReleaseNotes { pub (crate) fn run (self , sh : & Shell) -> anyhow :: Result < () > { let asciidoc = sh . read_file (& self . changelog) ? ; let mut markdown = notes :: convert_asciidoc_to_markdown (std :: io :: Cursor :: new (& asciidoc)) ? ; if ! markdown . starts_with ("# Changelog") { bail ! ("changelog Markdown should start with `# Changelog`") ; } const NEWLINES : & str = "\n\n" ; let Some (idx) = markdown . find (NEWLINES) else { bail ! ("missing newlines after changelog title") ; } ; markdown . replace_range (0 .. idx + NEWLINES . len () , "") ; let file_name = check_file_name (self . changelog) ? ; let tag_name = & file_name [0 .. 10] ; let original_changelog_url = create_original_changelog_url (& file_name) ; let additional_paragraph = format ! ("\nSee also the [changelog post]({original_changelog_url}).") ; markdown . push_str (& additional_paragraph) ; if self . dry_run { println ! ("{markdown}") ; } else { update_release (sh , tag_name , & markdown) ? ; } Ok (()) } }
};
}
