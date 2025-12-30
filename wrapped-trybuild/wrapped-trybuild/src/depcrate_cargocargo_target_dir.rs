// Generated macro for cargo_target_dir (function)
macro_rules! Depcrate_cargocargo_target_dir {
() => {
// Module: crate::cargo
// Provides: {"cargo_target_dir"}
// Dependencies: {}
fn cargo_target_dir (project : & Project) -> impl Iterator < Item = (& 'static str , PathBuf) > { iter :: once (("CARGO_TARGET_DIR" , path ! (project . target_dir / "tests" / "trybuild") ,)) }
};
}
