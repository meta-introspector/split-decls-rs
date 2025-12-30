// Generated macro for test (module)
macro_rules! Depcrate_unix_apple_cputest {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: * ; use std :: process :: Command ; # [test] fn check_vendor_and_brand () { let child = Command :: new ("sysctl") . arg ("-a") . output () . expect ("Failed to start command...") ; assert ! (child . status . success ()) ; let stdout = String :: from_utf8 (child . stdout) . expect ("Not valid UTF8") ; let sys = System :: new_with_specifics (crate :: RefreshKind :: nothing () . with_cpu (CpuRefreshKind :: nothing () . with_cpu_usage ()) ,) ; let cpus = sys . cpus () ; assert ! (! cpus . is_empty () , "no CPU found") ; if let Some (line) = stdout . lines () . find (| l | l . contains ("machdep.cpu.vendor")) { let sysctl_value = line . split (':') . nth (1) . unwrap () ; assert_eq ! (cpus [0] . vendor_id () , sysctl_value . trim ()) ; } if let Some (line) = stdout . lines () . find (| l | l . contains ("machdep.cpu.brand_string")) { let sysctl_value = line . split (':') . nth (1) . unwrap () ; assert_eq ! (cpus [0] . brand () , sysctl_value . trim ()) ; } } }
};
}
