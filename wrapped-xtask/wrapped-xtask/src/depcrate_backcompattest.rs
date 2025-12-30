// Generated macro for test (function)
macro_rules! Depcrate_backcompattest {
() => {
// Module: crate::backcompat
// Provides: {"test"}
// Dependencies: {}
pub fn test () { if DISABLED { println ! ("⚠️  backcompat (DISABLED)") ; return ; } println ! ("🧪 backcompat") ; println ! ("building old qemu-run.. (git revision: {REVISION_UNDER_TEST})") ; let qemu_run = match QemuRun :: build () { Ok (qemu_run) => qemu_run , Err (e) => { eprintln ! ("error building old qemu-run: {e}") ; ALL_ERRORS . lock () . unwrap () . push ("backcompat (building qemu-run)" . to_string ()) ; return ; } } ; for snapshot_test in all_snapshot_tests () { let feature = match snapshot_test { "alloc" => "alloc" , "net" => "ip_in_core" , _ => "" , } ; super :: do_test (| | qemu_run . run_snapshot (snapshot_test , feature) , "backcompat (see xtask/src/backcompat.rs for FIXME instructions)" ,) ; } }
};
}
