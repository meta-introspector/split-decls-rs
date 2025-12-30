// Generated macro for impl_80 (impl)
macro_rules! Depcrate_runnerimpl_80 {
() => {
// Module: crate::runner
// Provides: {"impl_80"}
// Dependencies: {}
impl SpawnStatus { fn is_ok (& self) -> bool { match self { Self :: Ok | Self :: Skipped => true , Self :: Failure (_) | Self :: Expected (_) => false , } } fn summary (& self) -> impl std :: fmt :: Display { let palette = snapbox :: report :: Palette :: color () ; match self { Self :: Ok => palette . info ("ok") , Self :: Skipped => palette . warn ("ignored") , Self :: Failure (_) | Self :: Expected (_) => palette . error ("failed") , } } }
};
}
