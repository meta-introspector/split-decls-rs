// Generated macro for cli (function)
macro_rules! Depcratecli {
() => {
// Module: crate
// Provides: {"cli"}
// Dependencies: {}
pub fn cli () -> clap :: Command { clap :: Command :: new ("xtask-spellcheck") . arg (Arg :: new ("color") . long ("color") . help ("Coloring: auto, always, never") . action (ArgAction :: Set) . value_name ("WHEN") . global (true) ,) . arg (Arg :: new ("quiet") . long ("quiet") . short ('q') . help ("Do not print cargo log messages") . action (ArgAction :: SetTrue) . global (true) ,) . arg (Arg :: new ("verbose") . long ("verbose") . short ('v') . help ("Use verbose output (-vv very verbose/build.rs output)") . action (ArgAction :: Count) . global (true) ,) . arg (Arg :: new ("write-changes") . long ("write-changes") . short ('w') . help ("Write fixes out") . action (ArgAction :: SetTrue) . global (true) ,) }
};
}
