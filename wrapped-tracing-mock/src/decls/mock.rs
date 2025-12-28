macro_rules! deps {
    () => {
        MockLayer!();
        MockSubscriber!();
        MockLayerBuilder!();
        Expect!();
    };
}

macro_rules! mock {
    () => {
        deps!();
        # [doc = " Create a [`MockLayerBuilder`] used to construct a"] # [doc = " [`MockLayer`]."] # [doc = ""] # [doc = " For additional information and examples, see the [`layer`]"] # [doc = " module and [`MockLayerBuilder`] documentation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tracing_mock::{expect, layer};"] # [doc = " use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};"] # [doc = ""] # [doc = " let span = expect::span()"] # [doc = "     .named(\"my_span\");"] # [doc = " let (layer, handle) = layer::mock()"] # [doc = "     // Enter a matching span"] # [doc = "     .enter(&span)"] # [doc = "     // Record an event with message \"collect parting message\""] # [doc = "     .event(expect::event().with_fields(expect::msg(\"say hello\")))"] # [doc = "     // Exit a matching span"] # [doc = "     .exit(&span)"] # [doc = "     // Expect no further messages to be recorded"] # [doc = "     .only()"] # [doc = "     // Return the subscriber and handle"] # [doc = "     .run_with_handle();"] # [doc = ""] # [doc = " // Use `set_default` to apply the `MockSubscriber` until the end"] # [doc = " // of the current scope (when the guard `_subscriber` is dropped)."] # [doc = " let _subscriber =  tracing_subscriber::registry()"] # [doc = "     .with(layer.with_filter(tracing_subscriber::filter::filter_fn(move |_meta| true)))"] # [doc = "     .set_default();"] # [doc = ""] # [doc = " {"] # [doc = "     let span = tracing::trace_span!("] # [doc = "         \"my_span\","] # [doc = "         greeting = \"hello world\","] # [doc = "     );"] # [doc = ""] # [doc = "     let _guard = span.enter();"] # [doc = "     tracing::info!(\"say hello\");"] # [doc = " }"] # [doc = ""] # [doc = " // Use the handle to check the assertions. This line will panic if an"] # [doc = " // assertion is not met."] # [doc = " handle.assert_finished();"] # [doc = " ```"] # [doc = ""] # [doc = " [`layer`]: mod@crate::layer"] # [must_use] pub fn mock () -> MockLayerBuilder { MockLayerBuilder { expected : Default :: default () , name : std :: thread :: current () . name () . map (String :: from) . unwrap_or_default () , } }
    };
}

mock!()