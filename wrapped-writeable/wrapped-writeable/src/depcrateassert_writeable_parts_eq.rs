// Generated macro for assert_writeable_parts_eq (macro)
macro_rules! Depcrateassert_writeable_parts_eq {
() => {
// Module: crate
// Provides: {"assert_writeable_parts_eq"}
// Dependencies: {}
# [doc = " See [`assert_writeable_eq`]."] # [macro_export] # [cfg (feature = "alloc")] macro_rules ! assert_writeable_parts_eq { ($ actual_writeable : expr , $ expected_str : expr , $ expected_parts : expr $ (,) ?) => { $ crate :: assert_writeable_parts_eq ! ($ actual_writeable , $ expected_str , $ expected_parts , "") } ; ($ actual_writeable : expr , $ expected_str : expr , $ expected_parts : expr , $ ($ arg : tt) +) => { { let actual_parts = $ crate :: assert_writeable_eq ! (@ internal , $ actual_writeable , $ expected_str , $ ($ arg) *) ; assert_eq ! (actual_parts , $ expected_parts , $ ($ arg) +) ; } } ; }
};
}
