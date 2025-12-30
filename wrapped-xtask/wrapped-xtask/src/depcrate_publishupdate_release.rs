// Generated macro for update_release (function)
macro_rules! Depcrate_publishupdate_release {
() => {
// Module: crate::publish
// Provides: {"update_release"}
// Dependencies: {}
fn update_release (sh : & Shell , tag_name : & str , release_notes : & str) -> anyhow :: Result < () > { let token = match env :: var ("GITHUB_TOKEN") { Ok (token) => token , Err (_) => bail ! ("Please obtain a personal access token from https://github.com/settings/tokens and set the `GITHUB_TOKEN` environment variable.") , } ; let accept = "Accept: application/vnd.github+json" ; let authorization = format ! ("Authorization: Bearer {token}") ; let api_version = "X-GitHub-Api-Version: 2022-11-28" ; let release_url = "https://api.github.com/repos/rust-lang/rust-analyzer/releases" ; let release_json = cmd ! (sh , "curl -sf -H {accept} -H {authorization} -H {api_version} {release_url}/tags/{tag_name}") . read () ? ; let release_id = cmd ! (sh , "jq .id") . stdin (release_json) . read () ? ; let mut patch = String :: new () ; write_json :: object (& mut patch) . string ("tag_name" , tag_name) . string ("target_commitish" , "master") . string ("name" , tag_name) . string ("body" , release_notes) . bool ("draft" , false) . bool ("prerelease" , false) ; let _ = cmd ! (sh , "curl -sf -X PATCH -H {accept} -H {authorization} -H {api_version} {release_url}/{release_id} -d {patch}") . read () ? ; Ok (()) }
};
}
