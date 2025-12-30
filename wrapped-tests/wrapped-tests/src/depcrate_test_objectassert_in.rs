// Generated macro for assert_in (macro)
macro_rules! Depcrate_test_objectassert_in {
() => {
// Module: crate::test_object
// Provides: {"assert_in"}
// Dependencies: {}
macro_rules ! assert_in { ($ item : expr , $ lst : expr) => { { let mut found = false ; for & x in $ lst . iter () { if x == $ item { found = true ; } } assert ! (found , "Did not find {} in {}" , stringify ! ($ item) , stringify ! ($ lst) ,) ; } } ; }
};
}
