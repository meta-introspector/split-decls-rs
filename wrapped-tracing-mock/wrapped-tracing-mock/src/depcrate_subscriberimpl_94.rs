// Generated macro for impl_94 (impl)
macro_rules! Depcrate_subscriberimpl_94 {
() => {
// Module: crate::subscriber
// Provides: {"impl_94"}
// Dependencies: {}
impl MockHandle { # [cfg (feature = "tracing-subscriber")] pub (crate) fn new (expected : Arc < Mutex < VecDeque < Expect > > > , name : String) -> Self { Self (expected , name) } # [doc = " Checks the expectations which were set on the"] # [doc = " [`MockSubscriber`]."] # [doc = ""] # [doc = " Calling `assert_finished` is usually the final part of a test."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This method will panic if any of the provided expectations are"] # [doc = " not met."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tracing_mock::{expect, subscriber};"] # [doc = ""] # [doc = " let (subscriber, handle) = subscriber::mock()"] # [doc = "     .event(expect::event())"] # [doc = "     .run_with_handle();"] # [doc = ""] # [doc = " tracing::subscriber::with_default(subscriber, || {"] # [doc = "     tracing::info!(\"a\");"] # [doc = " });"] # [doc = ""] # [doc = " // Check assertions set on the mock subscriber"] # [doc = " handle.assert_finished();"] # [doc = " ```"] pub fn assert_finished (& self) { if let Ok (ref expected) = self . 0 . lock () { assert ! (! expected . iter () . any (| thing | thing != & Expect :: Nothing) , "\n[{}] more notifications expected: {:#?}" , self . 1 , ** expected) ; } } }
};
}
