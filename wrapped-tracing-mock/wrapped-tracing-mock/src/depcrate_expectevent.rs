// Generated macro for event (function)
macro_rules! Depcrate_expectevent {
() => {
// Module: crate::expect
// Provides: {"event"}
// Dependencies: {}
# [doc = " Create a new [`ExpectedEvent`]."] # [doc = ""] # [doc = " For details on how to add additional assertions to the expected"] # [doc = " event, see the [`event`] module and the [`ExpectedEvent`] struct."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tracing_mock::{expect, subscriber};"] # [doc = ""] # [doc = " let (subscriber, handle) = subscriber::mock()"] # [doc = "     .event(expect::event())"] # [doc = "     .run_with_handle();"] # [doc = ""] # [doc = " tracing::subscriber::with_default(subscriber, || {"] # [doc = "     tracing::info!(field.name = \"field_value\");"] # [doc = " });"] # [doc = ""] # [doc = " handle.assert_finished();"] # [doc = " ```"] # [doc = ""] # [doc = " If we expect an event and instead record something else, the test"] # [doc = " will fail:"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use tracing_mock::{expect, subscriber};"] # [doc = ""] # [doc = " let (subscriber, handle) = subscriber::mock()"] # [doc = "     .event(expect::event())"] # [doc = "     .run_with_handle();"] # [doc = ""] # [doc = " tracing::subscriber::with_default(subscriber, || {"] # [doc = "     let span = tracing::info_span!(\"span\");"] # [doc = "     let _guard = span.enter();"] # [doc = " });"] # [doc = ""] # [doc = " handle.assert_finished();"] # [doc = " ```"] pub fn event () -> ExpectedEvent { ExpectedEvent { .. Default :: default () } }
};
}
