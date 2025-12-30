// Generated macro for cargo (function)
macro_rules! Depcrate_cargocargo {
() => {
// Module: crate::cargo
// Provides: {"cargo"}
// Dependencies: {}
fn cargo (project : & Project) -> Command { let mut cmd = raw_cargo () ; cmd . current_dir (& project . dir) ; cmd . envs (cargo_target_dir (project)) ; cmd . env_remove ("RUSTFLAGS") ; cmd . env ("CARGO_INCREMENTAL" , "0") ; cmd . arg ("--offline") ; let rustflags = rustflags :: toml () ; cmd . arg (format ! ("--config=build.rustflags={rustflags}")) ; cmd . arg (format ! ("--config=target.{TARGET}.rustflags={rustflags}")) ; cmd }
};
}
