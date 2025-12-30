// Generated macro for checkout_ws (function)
macro_rules! Depcrate_xtaskcheckout_ws {
() => {
// Module: crate::xtask
// Provides: {"checkout_ws"}
// Dependencies: {}
# [doc = " Checkouts a temporary workspace to do further version comparisons."] fn checkout_ws < 'gctx , 'a > (ws : & Workspace < 'gctx > , repo : & 'a git2 :: Repository , referenced_commit : & git2 :: Commit < 'a > ,) -> CargoResult < Workspace < 'gctx > > { let repo_path = repo . path () . as_os_str () . to_str () . unwrap () ; let short_id = & referenced_commit . id () . to_string () [.. 7] ; let checkout_path = ws . target_dir () . join (format ! ("cargo-{short_id}")) ; let checkout_path = checkout_path . as_path_unlocked () ; let _ = fs :: remove_dir_all (checkout_path) ; let new_repo = git2 :: build :: RepoBuilder :: new () . clone_local (git2 :: build :: CloneLocal :: Local) . clone (repo_path , checkout_path) ? ; let obj = new_repo . find_object (referenced_commit . id () , None) ? ; new_repo . reset (& obj , git2 :: ResetType :: Hard , None) ? ; Workspace :: new (& checkout_path . join ("Cargo.toml") , ws . gctx ()) }
};
}
