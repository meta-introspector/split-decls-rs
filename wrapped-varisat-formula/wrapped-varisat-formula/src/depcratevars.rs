// Generated macro for vars (macro)
macro_rules! Depcratevars {
() => {
// Module: crate
// Provides: {"vars"}
// Dependencies: {}
# [doc = " Shortcut for tests"] # [cfg (any (test , feature = "internal-testing"))] # [doc (hidden)] # [macro_export] macro_rules ! vars { ($ ($ x : expr) ,*) => { [$ ($ crate :: var ! ($ x)) ,*] } ; ($ ($ x : expr) ,* ,) => { $ crate :: vars ! [$ ($ x) ,*] } ; }
};
}
