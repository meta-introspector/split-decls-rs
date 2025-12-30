// Generated macro for send_sync_traits (function)
macro_rules! Depcrate_tests_recursivesend_sync_traits {
() => {
// Module: crate::tests::recursive
// Provides: {"send_sync_traits"}
// Dependencies: {}
# [test] fn send_sync_traits () { use crate :: { FilterEntry , IntoIter } ; fn assert_send < T : Send > () { } fn assert_sync < T : Sync > () { } assert_send :: < WalkDir > () ; assert_sync :: < WalkDir > () ; assert_send :: < IntoIter > () ; assert_sync :: < IntoIter > () ; assert_send :: < FilterEntry < IntoIter , u8 > > () ; assert_sync :: < FilterEntry < IntoIter , u8 > > () ; }
};
}
