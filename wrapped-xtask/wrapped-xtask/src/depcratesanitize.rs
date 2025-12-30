// Generated macro for sanitize (function)
macro_rules! Depcratesanitize {
() => {
// Module: crate
// Provides: {"sanitize"}
// Dependencies: {}
fn sanitize (cmd : & str) -> Command { let cmd = { let exe = format ! ("{cmd}{}" , env :: consts :: EXE_SUFFIX) ; let mut cargo_home = home :: cargo_home () . unwrap () ; cargo_home . push ("bin") ; cargo_home . push (& exe) ; if cargo_home . exists () { cargo_home } else { PathBuf :: from (exe) } } ; let mut cmd = Command :: new (cmd) ; cmd . current_dir (project_root ()) ; cmd . env_remove ("LD_LIBRARY_PATH") ; env :: vars () . filter (| (key , _value) | { key . starts_with ("CARGO") && ! key . starts_with ("CARGO_HOME") || key . starts_with ("RUST") && ! key . starts_with ("RUSTUP_HOME") }) . for_each (| (key , _value) | { cmd . env_remove (& key) ; }) ; cmd }
};
}
