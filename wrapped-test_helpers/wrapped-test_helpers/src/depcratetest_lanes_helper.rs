// Generated macro for test_lanes_helper (macro)
macro_rules! Depcratetest_lanes_helper {
() => {
// Module: crate
// Provides: {"test_lanes_helper"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! test_lanes_helper { ($ ($ (# [$ meta : meta]) * $ fn_name : ident $ lanes : literal ;) +) => { $ (# [test] $ (# [$ meta]) * fn $ fn_name () { implementation ::<$ lanes > () ; }) + } ; ($ (# [$ meta : meta]) +; $ ($ (# [$ meta_before : meta]) + $ fn_name_before : ident $ lanes_before : literal ;) * $ fn_name : ident $ lanes : literal ; $ ($ fn_name_rest : ident $ lanes_rest : literal ;) *) => { $ crate :: test_lanes_helper ! ($ (# [$ meta]) +; $ ($ (# [$ meta_before]) + $ fn_name_before $ lanes_before ;) * $ (# [$ meta]) + $ fn_name $ lanes ; $ ($ fn_name_rest $ lanes_rest ;) *) ; } ; ($ (# [$ meta_ignored : meta]) +; $ ($ (# [$ meta : meta]) + $ fn_name : ident $ lanes : literal ;) +) => { $ crate :: test_lanes_helper ! ($ ($ (# [$ meta]) + $ fn_name $ lanes ;) +) ; } ; }
};
}
