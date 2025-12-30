// Generated macro for other_2 (other)
macro_rules! Depcrateother_2 {
() => {
// Module: crate
// Provides: {"other_2"}
// Dependencies: {}
pub macro link_dylib { ($ library : literal $ abi : literal $ ($ link_name : literal) ? $ (# [$ doc : meta]) ? fn $ ($ function : tt) *) => (# [link (name = "kernel32")] unsafe extern $ abi { $ (# [link_name =$ link_name]) ? pub fn $ ($ function) *; }) }
};
}
