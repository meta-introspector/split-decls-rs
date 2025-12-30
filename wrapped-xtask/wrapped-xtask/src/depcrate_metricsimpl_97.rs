// Generated macro for impl_97 (impl)
macro_rules! Depcrate_metricsimpl_97 {
() => {
// Module: crate::metrics
// Provides: {"impl_97"}
// Dependencies: {}
impl Host { fn unknown () -> Host { Host { os : "unknown" . into () , cpu : "unknown" . into () , mem : "unknown" . into () } } fn new (sh : & Shell) -> anyhow :: Result < Host > { if cfg ! (not (target_os = "linux")) { return Ok (Host :: unknown ()) ; } let os = read_field (sh , "/etc/os-release" , "PRETTY_NAME=") ? . trim_matches ('"') . to_owned () ; let cpu = read_field (sh , "/proc/cpuinfo" , "model name") ? . trim_start_matches (':') . trim () . to_owned () ; let mem = read_field (sh , "/proc/meminfo" , "MemTotal:") ? ; return Ok (Host { os , cpu , mem }) ; fn read_field (sh : & Shell , path : & str , field : & str) -> anyhow :: Result < String > { let text = sh . read_file (path) ? ; text . lines () . find_map (| it | it . strip_prefix (field)) . map (| it | it . trim () . to_owned ()) . ok_or_else (| | format_err ! ("can't parse {}" , path)) } } fn to_json (& self , mut obj : write_json :: Object < '_ >) { obj . string ("os" , & self . os) . string ("cpu" , & self . cpu) . string ("mem" , & self . mem) ; } }
};
}
