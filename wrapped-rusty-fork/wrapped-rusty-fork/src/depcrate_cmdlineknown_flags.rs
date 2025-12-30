// Generated macro for KNOWN_FLAGS (static)
macro_rules! Depcrate_cmdlineKNOWN_FLAGS {
() => {
// Module: crate::cmdline
// Provides: {"KNOWN_FLAGS"}
// Dependencies: {}
# [doc = " Table of all flags in the 2025-10-04 nightly build."] # [doc = ""] # [doc = " A number of these that affect output are dropped because we append our own"] # [doc = " options."] static KNOWN_FLAGS : & [(& str , FlagType)] = & [("--bench" , FlagType :: Pass (false)) , ("--color" , FlagType :: Pass (true)) , ("--ensure-time" , FlagType :: Drop (false)) , ("--exact" , FlagType :: Drop (false)) , ("--exclude-should-panic" , FlagType :: Pass (false)) , ("--fail-fast" , FlagType :: Drop (false)) , ("--force-run-in-process" , FlagType :: Pass (false)) , ("--format" , FlagType :: Drop (true)) , ("--help" , FlagType :: Error ("Tests run but --help passed to process?")) , ("--ignored" , FlagType :: Pass (false)) , ("--include-ignored" , FlagType :: Pass (false)) , ("--list" , FlagType :: Error ("Tests run but --list passed to process?")) , ("--logfile" , FlagType :: Drop (true)) , ("--nocapture" , FlagType :: Drop (true)) , ("--quiet" , FlagType :: Drop (false)) , ("--report-time" , FlagType :: Drop (true)) , ("--show-output" , FlagType :: Pass (false)) , ("--shuffle" , FlagType :: Drop (false)) , ("--shuffle-seed" , FlagType :: Drop (true)) , ("--skip" , FlagType :: Drop (true)) , ("--test" , FlagType :: Pass (false)) , ("--test-threads" , FlagType :: Drop (true)) , ("-Z" , FlagType :: Pass (true)) , ("-h" , FlagType :: Error ("Tests run but -h passed to process?")) , ("-q" , FlagType :: Drop (false)) ,] ;
};
}
