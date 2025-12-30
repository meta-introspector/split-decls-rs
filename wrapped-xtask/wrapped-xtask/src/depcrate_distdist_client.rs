// Generated macro for dist_client (function)
macro_rules! Depcrate_distdist_client {
() => {
// Module: crate::dist
// Provides: {"dist_client"}
// Dependencies: {}
fn dist_client (sh : & Shell , version : & str , release_tag : & str , target : & Target ,) -> anyhow :: Result < () > { let bundle_path = Path :: new ("editors") . join ("code") . join ("server") ; sh . create_dir (& bundle_path) ? ; sh . copy_file (& target . server_path , & bundle_path) ? ; if let Some (symbols_path) = & target . symbols_path { sh . copy_file (symbols_path , & bundle_path) ? ; } let _d = sh . push_dir ("./editors/code") ; let mut patch = Patch :: new (sh , "./package.json") ? ; patch . replace (& format ! (r#""version": "{VERSION_DEV}.0-dev""#) , & format ! (r#""version": "{version}""#) ,) . replace (r#""releaseTag": null"# , & format ! (r#""releaseTag": "{release_tag}""#)) . replace (r#""title": "$generated-start""# , "") . replace (r#""title": "$generated-end""# , "") . replace (r#""enabledApiProposals": [],"# , r#""#) ; patch . commit (sh) ? ; Ok (()) }
};
}
