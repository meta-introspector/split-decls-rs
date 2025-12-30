// Generated macro for impl_15 (impl)
macro_rules! Depcrate_test_mock_storageimpl_15 {
() => {
// Module: crate::test::mock_storage
// Provides: {"impl_15"}
// Dependencies: {}
# [async_trait] impl Storage for MockStorage { async fn get (& self , _key : & str) -> Result < Cache > { if let Some (delay) = self . delay { sleep (delay) . await ; } let next = self . rx . lock () . await . try_next () . unwrap () ; next . expect ("MockStorage get called but no get results available") } async fn put (& self , _key : & str , _entry : CacheWrite) -> Result < Duration > { Ok (if let Some (delay) = self . delay { sleep (delay) . await ; delay } else { Duration :: from_secs (0) }) } fn location (& self) -> String { "Mock Storage" . to_string () } async fn current_size (& self) -> Result < Option < u64 > > { Ok (None) } async fn max_size (& self) -> Result < Option < u64 > > { Ok (None) } fn preprocessor_cache_mode_config (& self) -> PreprocessorCacheModeConfig { PreprocessorCacheModeConfig { use_preprocessor_cache_mode : self . preprocessor_cache_mode , .. Default :: default () } } }
};
}
