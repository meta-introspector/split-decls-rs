// Generated macro for reuse_pin_box (function)
macro_rules! Depcrate_sync_reusable_boxreuse_pin_box {
() => {
// Module: crate::sync::reusable_box
// Provides: {"reuse_pin_box"}
// Dependencies: {}
fn reuse_pin_box < T : ? Sized , U , O , F > (boxed : Pin < Box < T > > , new_value : U , callback : F) -> Result < O , U > where F : FnOnce (Box < U >) -> O , { let layout = Layout :: for_value :: < T > (& * boxed) ; if layout != Layout :: new :: < U > () { return Err (new_value) ; } let raw : * mut T = Box :: into_raw (unsafe { Pin :: into_inner_unchecked (boxed) }) ; let guard = CallOnDrop :: new (| | { let raw : * mut U = raw . cast :: < U > () ; unsafe { raw . write (new_value) } ; let boxed = unsafe { Box :: from_raw (raw) } ; callback (boxed) }) ; unsafe { ptr :: drop_in_place (raw) } ; Ok (guard . call ()) }
};
}
