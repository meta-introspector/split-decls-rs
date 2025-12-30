// Generated macro for init_logging (function)
macro_rules! Depcrateinit_logging {
() => {
// Module: crate
// Provides: {"init_logging"}
// Dependencies: {}
fn init_logging () { let format = | buf : & mut fmt :: Formatter , record : & Record | { if record . level () == Level :: Info { writeln ! (buf , "c {}" , record . args ()) } else { writeln ! (buf , "c {}: {}" , record . level () , record . args ()) } } ; let mut builder = Builder :: new () ; builder . target (Target :: Stdout) . format (format) . filter (None , LevelFilter :: Info) ; if let Ok (ref env_var) = env :: var ("VARISAT_LOG") { builder . parse_filters (env_var) ; } builder . init () ; }
};
}
