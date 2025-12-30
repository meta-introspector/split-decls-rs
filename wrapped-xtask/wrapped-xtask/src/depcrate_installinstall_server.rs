// Generated macro for install_server (function)
macro_rules! Depcrate_installinstall_server {
() => {
// Module: crate::install
// Provides: {"install_server"}
// Dependencies: {}
fn install_server (sh : & Shell , opts : ServerOpt) -> anyhow :: Result < () > { let features = & opts . to_features () ; let profile = if opts . dev_rel { "dev-rel" } else { "release" } ; let mut install_cmd = cmd ! (sh , "cargo install --path crates/rust-analyzer --profile={profile} --locked --force --features force-always-assert {features...}") ; if let Some (train_crate) = opts . pgo { let target = detect_target (sh) ; let build_cmd = cmd ! (sh , "cargo build --manifest-path ./crates/rust-analyzer/Cargo.toml --bin rust-analyzer --target {target} --profile={profile} --locked {features...}") ; let profile = crate :: pgo :: gather_pgo_profile (sh , build_cmd , & target , train_crate) ? ; install_cmd = crate :: pgo :: apply_pgo_to_cmd (install_cmd , & profile) ; } install_cmd . run () ? ; Ok (()) }
};
}
