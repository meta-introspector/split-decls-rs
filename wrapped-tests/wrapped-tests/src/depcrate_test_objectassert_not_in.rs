// Generated macro for assert_not_in (macro)
macro_rules! Depcrate_test_objectassert_not_in {
() => {
// Module: crate::test_object
// Provides: {"assert_not_in"}
// Dependencies: {}
macro_rules ! assert_not_in { ($ item : expr , $ lst : expr) => { { let mut found = false ; for & x in $ lst . iter () { if x == $ item { found = true ; } } assert ! (! found , "Found {} in {}" , stringify ! ($ item) , stringify ! ($ lst) ,) ; } } ; }
};
}
