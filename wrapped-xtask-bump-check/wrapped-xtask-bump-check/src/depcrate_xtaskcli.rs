// Generated macro for cli (function)
macro_rules! Depcrate_xtaskcli {
() => {
// Module: crate::xtask
// Provides: {"cli"}
// Dependencies: {}
pub fn cli () -> clap :: Command { clap :: Command :: new ("xtask-bump-check") . arg (opt ("verbose" , "Use verbose output (-vv very verbose/build.rs output)" ,) . short ('v') . action (ArgAction :: Count) . global (true) ,) . arg (flag ("quiet" , "Do not print cargo log messages") . short ('q') . global (true) ,) . arg (opt ("color" , "Coloring: auto, always, never") . value_name ("WHEN") . global (true) ,) . arg (opt ("base-rev" , "Git revision to lookup for a baseline")) . arg (opt ("head-rev" , "Git revision with changes")) . arg (flag ("frozen" , "Require Cargo.lock and cache to be up-to-date") . global (true)) . arg (flag ("locked" , "Require Cargo.lock to be up-to-date") . global (true)) . arg (flag ("offline" , "Run without accessing the network") . global (true)) . arg (multi_opt ("config" , "KEY=VALUE" , "Override a configuration value") . global (true)) . arg (flag ("github" , "Group output using GitHub's syntax")) . arg (Arg :: new ("unstable-features") . help ("Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details") . short ('Z') . value_name ("FLAG") . action (ArgAction :: Append) . global (true) ,) }
};
}
