// Generated macro for core_internal (macro)
macro_rules! Depcrate_serial_code_lockcore_internal {
() => {
// Module: crate::serial_code_lock
// Provides: {"core_internal"}
// Dependencies: {}
# [doc (hidden)] macro_rules ! core_internal { ($ names : ident) => { let unlocks : Vec < _ > = $ names . into_iter () . map (| name | { check_new_key (name) ; global_locks () . get (name) . expect ("key to be set") . get () . clone () }) . collect () ; let _guards : Vec < _ > = unlocks . iter () . map (| unlock | unlock . lock ()) . collect () ; } ; }
};
}
