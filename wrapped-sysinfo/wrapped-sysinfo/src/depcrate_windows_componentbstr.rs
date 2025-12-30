// Generated macro for bstr (macro)
macro_rules! Depcrate_windows_componentbstr {
() => {
// Module: crate::windows::component
// Provides: {"bstr"}
// Dependencies: {}
macro_rules ! bstr { ($ x : literal) => { { SysAllocString (w ! ($ x)) } } ; }
};
}
