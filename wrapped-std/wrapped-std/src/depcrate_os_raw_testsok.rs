// Generated macro for ok (macro)
macro_rules! Depcrate_os_raw_testsok {
() => {
// Module: crate::os::raw::tests
// Provides: {"ok"}
// Dependencies: {}
macro_rules ! ok { ($ ($ t : ident) *) => { $ (assert ! (TypeId :: of ::< libc ::$ t > () == TypeId :: of ::< raw ::$ t > () , "{} is wrong" , stringify ! ($ t)) ;) * } }
};
}
