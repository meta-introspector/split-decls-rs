// Generated macro for beta_and_stable_branch (function)
macro_rules! Depcrate_xtaskbeta_and_stable_branch {
() => {
// Module: crate::xtask
// Provides: {"beta_and_stable_branch"}
// Dependencies: {}
# [doc = " Get the current beta and stable branch in cargo repository."] # [doc = ""] # [doc = " Assumptions:"] # [doc = ""] # [doc = " * The repository contains the full history of `<remote>/rust-1.*.0` branches."] # [doc = " * The version part of `<remote>/rust-1.*.0` always ends with a zero."] # [doc = " * The maximum version is for beta channel, and the second one is for stable."] fn beta_and_stable_branch (repo : & git2 :: Repository) -> CargoResult < [git2 :: Branch < '_ > ; 2] > { let mut release_branches = Vec :: new () ; for branch in repo . branches (Some (git2 :: BranchType :: Remote)) ? { let (branch , _) = branch ? ; let name = branch . name () ? . unwrap () ; let Some ((_ , version)) = name . split_once ("/rust-") else { tracing :: trace ! ("branch `{name}` is not in the format of `<remote>/rust-<semver>`") ; continue ; } ; let Ok (version) = version . parse :: < semver :: Version > () else { tracing :: trace ! ("branch `{name}` is not a valid semver: `{version}`") ; continue ; } ; release_branches . push ((version , branch)) ; } release_branches . sort_unstable_by (| a , b | a . 0 . cmp (& b . 0)) ; release_branches . dedup_by (| a , b | a . 0 == b . 0) ; let beta = release_branches . pop () . unwrap () ; let stable = release_branches . pop () . unwrap () ; assert_eq ! (beta . 0 . major , 1) ; assert_eq ! (beta . 0 . patch , 0) ; assert_eq ! (stable . 0 . major , 1) ; assert_eq ! (stable . 0 . patch , 0) ; assert_ne ! (beta . 0 . minor , stable . 0 . minor) ; Ok ([beta . 1 , stable . 1]) }
};
}
