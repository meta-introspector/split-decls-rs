// Generated macro for span (function)
macro_rules! Depcrate_expectspan {
() => {
// Module: crate::expect
// Provides: {"span"}
// Dependencies: {}
# [doc = " Construct a new [`ExpectedSpan`]."] # [doc = ""] # [doc = " For details on how to add additional assertions to the expected"] # [doc = " span, see the [`span`] module and the [`ExpectedSpan`] and"] # [doc = " [`NewSpan`] structs."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tracing_mock::{expect, subscriber};"] # [doc = ""] # [doc = " let (subscriber, handle) = subscriber::mock()"] # [doc = "     .new_span(expect::span())"] # [doc = "     .enter(expect::span())"] # [doc = "     .run_with_handle();"] # [doc = ""] # [doc = " tracing::subscriber::with_default(subscriber, || {"] # [doc = "     let span = tracing::info_span!(\"span\");"] # [doc = "     let _guard = span.enter();"] # [doc = " });"] # [doc = ""] # [doc = " handle.assert_finished();"] # [doc = " ```"] # [doc = ""] # [doc = " If we expect to enter a span and instead record something else, the test"] # [doc = " will fail:"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use tracing_mock::{expect, subscriber};"] # [doc = ""] # [doc = " let (subscriber, handle) = subscriber::mock()"] # [doc = "     .enter(expect::span())"] # [doc = "     .run_with_handle();"] # [doc = ""] # [doc = " tracing::subscriber::with_default(subscriber, || {"] # [doc = "     tracing::info!(field.name = \"field_value\");"] # [doc = " });"] # [doc = ""] # [doc = " handle.assert_finished();"] # [doc = " ```"] pub fn span () -> ExpectedSpan { ExpectedSpan { .. Default :: default () } }
};
}
