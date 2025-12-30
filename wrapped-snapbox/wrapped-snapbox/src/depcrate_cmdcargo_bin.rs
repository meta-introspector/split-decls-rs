// Generated macro for cargo_bin (function)
macro_rules! Depcrate_cmdcargo_bin {
() => {
// Module: crate::cmd
// Provides: {"cargo_bin"}
// Dependencies: {}
# [doc = " Look up the path to a cargo-built binary within an integration test."] # [doc = ""] # [doc = " **NOTE:** Prefer [`cargo_bin!`] as this makes assumptions about cargo"] # [deprecated (since = "0.6.23" , note = "incompatible with a custom cargo build-dir, see instead `cmd::cargo_bin!`")] pub fn cargo_bin (name : & str) -> std :: path :: PathBuf { let file_name = format ! ("{}{}" , name , std :: env :: consts :: EXE_SUFFIX) ; let target_dir = target_dir () ; target_dir . join (file_name) }
};
}
