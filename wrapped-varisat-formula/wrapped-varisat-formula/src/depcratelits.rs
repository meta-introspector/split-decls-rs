// Generated macro for lits (macro)
macro_rules! Depcratelits {
() => {
// Module: crate
// Provides: {"lits"}
// Dependencies: {}
# [doc = " Shortcut for tests"] # [cfg (any (test , feature = "internal-testing"))] # [doc (hidden)] # [macro_export] macro_rules ! lits { ($ ($ x : expr) ,*) => { [$ ($ crate :: lit ! ($ x)) ,*] } ; ($ ($ x : expr) ,* ,) => { $ crate :: lits ! [$ ($ x) ,*] } ; }
};
}
