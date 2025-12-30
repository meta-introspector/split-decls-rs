// Generated macro for destroy (function)
macro_rules! Depcrate_closuredestroy {
() => {
// Module: crate::closure
// Provides: {"destroy"}
// Dependencies: {}
unsafe extern "C" fn destroy < T : ? Sized > (a : usize , b : usize) { if a == 0 { return ; } drop (mem :: transmute_copy :: < _ , Box < T > > (& (a , b))) ; }
};
}
