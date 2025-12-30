// Generated macro for get_referenced_commit (function)
macro_rules! Depcrate_xtaskget_referenced_commit {
() => {
// Module: crate::xtask
// Provides: {"get_referenced_commit"}
// Dependencies: {}
# [doc = " Gets the referenced commit to compare if version bump needed."] # [doc = ""] # [doc = " * When merging into nightly, check the version with beta branch"] # [doc = " * When merging into beta, check the version with stable branch"] # [doc = " * When merging into stable, check against crates.io registry directly"] fn get_referenced_commit < 'a > (repo : & 'a git2 :: Repository , base : & git2 :: Commit < 'a > ,) -> CargoResult < Option < git2 :: Commit < 'a > > > { let [beta , stable] = beta_and_stable_branch (repo) ? ; let rev_id = base . id () ; let stable_commit = stable . get () . peel_to_commit () ? ; let beta_commit = beta . get () . peel_to_commit () ? ; let referenced_commit = if rev_id == stable_commit . id () { None } else if rev_id == beta_commit . id () { tracing :: trace ! ("stable branch from `{}`" , stable . name () . unwrap () . unwrap ()) ; Some (stable_commit) } else { tracing :: trace ! ("beta branch from `{}`" , beta . name () . unwrap () . unwrap ()) ; Some (beta_commit) } ; Ok (referenced_commit) }
};
}
