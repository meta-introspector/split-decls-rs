// Generated macro for other_1 (other)
macro_rules! Depcrateother_1 {
() => {
// Module: crate
// Provides: {"other_1"}
// Dependencies: {}
pub macro link_raw_dylib { ($ library : literal $ abi : literal $ ($ link_name : literal) ? $ (# [$ doc : meta]) ? fn $ ($ function : tt) *) => (# [cfg_attr (not (target_arch = "x86") , link (name = $ library , kind = "raw-dylib" , modifiers = "+verbatim"))] # [cfg_attr (target_arch = "x86" , link (name = $ library , kind = "raw-dylib" , modifiers = "+verbatim" , import_name_type = "undecorated"))] unsafe extern $ abi { $ (# [link_name =$ link_name]) ? pub fn $ ($ function) *; }) }
};
}
