// Generated macro for impl_14 (impl)
macro_rules! Depcrate_test_mock_storageimpl_14 {
() => {
// Module: crate::test::mock_storage
// Provides: {"impl_14"}
// Dependencies: {}
impl MockStorage { # [doc = " Create a new `MockStorage`. if `delay` is `Some`, wait for that amount of time before returning from operations."] pub (crate) fn new (delay : Option < Duration > , preprocessor_cache_mode : bool) -> MockStorage { let (tx , rx) = mpsc :: unbounded () ; Self { tx , rx : Arc :: new (Mutex :: new (rx)) , delay , preprocessor_cache_mode , } } # [doc = " Queue up `res` to be returned as the next result from `Storage::get`."] pub (crate) fn next_get (& self , res : Result < Cache >) { self . tx . unbounded_send (res) . unwrap () ; } }
};
}
