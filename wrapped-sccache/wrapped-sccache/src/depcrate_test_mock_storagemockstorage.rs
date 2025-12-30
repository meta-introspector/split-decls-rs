// Generated macro for MockStorage (struct)
macro_rules! Depcrate_test_mock_storageMockStorage {
() => {
// Module: crate::test::mock_storage
// Provides: {"MockStorage"}
// Dependencies: {}
# [doc = " A mock `Storage` implementation."] pub struct MockStorage { rx : Arc < Mutex < mpsc :: UnboundedReceiver < Result < Cache > > > > , tx : mpsc :: UnboundedSender < Result < Cache > > , delay : Option < Duration > , preprocessor_cache_mode : bool , }
};
}
