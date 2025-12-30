// Generated macro for download_crate_for_training (function)
macro_rules! Depcrate_pgodownload_crate_for_training {
() => {
// Module: crate::pgo
// Provides: {"download_crate_for_training"}
// Dependencies: {}
# [doc = " Downloads a crate from GitHub, stores it into `pgo_dir` and returns a path to it."] fn download_crate_for_training (sh : & Shell , pgo_dir : & Path , repo : & str) -> anyhow :: Result < PathBuf > { let mut it = repo . splitn (2 , '@') ; let repo = it . next () . unwrap () ; let revision = it . next () ; let revision = if let Some (revision) = revision { & ["--branch" , revision] as & [& str] } else { & [] } ; let normalized_path = repo . replace ("/" , "-") ; let target_path = pgo_dir . join (normalized_path) ; cmd ! (sh , "git clone --depth 1 https://github.com/{repo} {revision...} {target_path}") . run () . with_context (| | "cannot download PGO training crate from {repo}") ? ; Ok (target_path) }
};
}
