// Generated macro for example (function)
macro_rules! Depcrate_combinator_debugexample {
() => {
// Module: crate::combinator::debug
// Provides: {"example"}
// Dependencies: {}
# [test] # [cfg (feature = "std")] # [cfg_attr (miri , ignore)] # [cfg (unix)] # [cfg (feature = "debug")] fn example () { use term_transcript :: { test :: TestConfig , ShellOptions } ; let path = snapbox :: cmd :: compile_example ("string" , ["--features=debug"]) . unwrap () ; let current_dir = path . parent () . unwrap () ; let cmd = path . file_name () . unwrap () ; let cmd = format ! ("./{}" , cmd . to_string_lossy ()) ; TestConfig :: new (ShellOptions :: default () . with_current_dir (current_dir) . with_env ("CLICOLOR_FORCE" , "1") ,) . test ("assets/trace.svg" , [format ! (r#"{cmd} '"abc"'"#) . as_str ()]) ; }
};
}
