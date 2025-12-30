// Generated macro for type_assert_failed (function)
macro_rules! Depcrate_tabletype_assert_failed {
() => {
// Module: crate::table
// Provides: {"type_assert_failed"}
// Dependencies: {}
# [doc = " This function is explicitly outlined to avoid debug machinery in the hot-path."] # [cold] # [inline (never)] fn type_assert_failed < T : 'static > (page : & Page) -> ! { panic ! ("page has slot type `{:?}` but `{:?}` was expected" , (page . slot_vtable . type_name) () , std :: any :: type_name ::< T > () ,) }
};
}
