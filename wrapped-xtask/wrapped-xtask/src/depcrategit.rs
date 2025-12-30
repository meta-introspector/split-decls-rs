// Generated macro for git (module)
macro_rules! Depcrategit {
() => {
// Module: crate
// Provides: {"git"}
// Dependencies: {}
pub mod git { use xshell :: { Shell , cmd } ; use super :: { Result , dry_run } ; pub fn current_branch (sh : & mut Shell) -> Result < String > { let res = cmd ! (sh , "git branch --show-current") . read () ? ; Ok (res) } pub fn tag_list (sh : & mut Shell) -> Result < Vec < String > > { let tags = cmd ! (sh , "git tag --list") . read () ? ; let res = tags . lines () . map (| it | it . trim () . to_string ()) . collect () ; Ok (res) } pub fn has_tag (tag : & str , sh : & mut Shell) -> Result < bool > { let res = tag_list (sh) ? . iter () . any (| it | it == tag) ; Ok (res) } pub fn tag (tag : & str , sh : & mut Shell) -> Result < () > { if dry_run () . is_some () { return Ok (()) ; } cmd ! (sh , "git tag {tag}") . run () ? ; Ok (()) } pub fn push_tags (sh : & mut Shell) -> Result < () > { if dry_run () . is_some () { return Ok (()) ; } cmd ! (sh , "git push --tags") . run () ? ; Ok (()) } }
};
}
