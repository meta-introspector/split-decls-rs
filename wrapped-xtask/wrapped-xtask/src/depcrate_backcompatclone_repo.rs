// Generated macro for clone_repo (function)
macro_rules! Depcrate_backcompatclone_repo {
() => {
// Module: crate::backcompat
// Provides: {"clone_repo"}
// Dependencies: {}
fn clone_repo (tempdir : & Path) -> anyhow :: Result < () > { let repo_path = Path :: new (env ! ("CARGO_MANIFEST_DIR")) . parent () . unwrap () ; run_silently (Command :: new ("git") . arg ("clone") . arg (repo_path) . arg (".") . current_dir (tempdir) , | | anyhow ! ("`git clone` failed") ,) ? ; run_silently (Command :: new ("git") . args (["reset" , "--hard" , REVISION_UNDER_TEST]) . current_dir (tempdir) , | | anyhow ! ("`git reset` failed") ,) ? ; Ok (()) }
};
}
