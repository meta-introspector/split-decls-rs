// Generated macro for assert_try_writeable_parts_eq (macro)
macro_rules! Depcrate_try_writeableassert_try_writeable_parts_eq {
() => {
// Module: crate::try_writeable
// Provides: {"assert_try_writeable_parts_eq"}
// Dependencies: {}
# [doc = " See [`assert_try_writeable_eq`]."] # [macro_export] macro_rules ! assert_try_writeable_parts_eq { ($ actual_writeable : expr , $ expected_str : expr , $ expected_parts : expr $ (,) ?) => { $ crate :: assert_try_writeable_parts_eq ! ($ actual_writeable , $ expected_str , Ok (()) , $ expected_parts) } ; ($ actual_writeable : expr , $ expected_str : expr , $ expected_result : expr , $ expected_parts : expr $ (,) ?) => { $ crate :: assert_try_writeable_parts_eq ! ($ actual_writeable , $ expected_str , $ expected_result , $ expected_parts , "") } ; ($ actual_writeable : expr , $ expected_str : expr , $ expected_result : expr , $ expected_parts : expr , $ ($ arg : tt) +) => { { let actual_parts = $ crate :: assert_try_writeable_eq ! (@ internal , $ actual_writeable , $ expected_str , $ expected_result , $ ($ arg) *) ; assert_eq ! (actual_parts , $ expected_parts , $ ($ arg) +) ; } } ; }
};
}
