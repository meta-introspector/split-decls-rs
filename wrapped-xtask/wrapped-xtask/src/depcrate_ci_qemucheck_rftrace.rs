// Generated macro for check_rftrace (function)
macro_rules! Depcrate_ci_qemucheck_rftrace {
() => {
// Module: crate::ci::qemu
// Provides: {"check_rftrace"}
// Dependencies: {}
fn check_rftrace (image : & Path) -> Result < () > { let sh = crate :: sh () ? ; let image_name = image . file_name () . unwrap () . to_str () . unwrap () ; let nm = crate :: binutil ("nm") . unwrap () ; let symbols = cmd ! (sh , "{nm} --numeric-sort {image}") . output () ? . stdout ; sh . write_file (format ! ("shared/tracedir/{image_name}.sym") , symbols) ? ; let replay = cmd ! (sh , "uftrace replay --data=shared/tracedir --output-fields=tid") . read () ? ; eprintln ! ("[CI] replay: {replay}") ; let expected = fs :: read_to_string ("xtask/src/ci/rftrace.snap") ? ; if ! replay . starts_with (& expected) { eprintln ! ("[CI] expected: {expected}") ; bail ! ("rftrace output does not match snapshot") ; } eprintln ! ("[CI] replay matches snapshot") ; Ok (()) }
};
}
