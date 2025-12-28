macro_rules! deps {
    () => {
        MockLayerBuilder!();
    };
}

macro_rules! named {
    () => {
        deps!();
        # [doc = " Create a [`MockLayerBuilder`] with a name already set."] # [doc = ""] # [doc = " This constructor is equivalent to calling"] # [doc = " [`MockLayerBuilder::named`] in the following way\""] # [doc = " `layer::mock().named(name)`."] # [doc = ""] # [doc = " For additional information and examples, see the [`layer`]"] # [doc = " module and [`MockLayerBuilder`] documentation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " The example from [`MockLayerBuilder::named`] could be rewritten as:"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use tracing_mock::{expect, layer};"] # [doc = " use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};"] # [doc = ""] # [doc = " let (layer_1, handle_1) = layer::named(\"subscriber-1\")"] # [doc = "     .event(expect::event())"] # [doc = "     .run_with_handle();"] # [doc = ""] # [doc = " let (layer_2, handle_2) = layer::named(\"subscriber-2\")"] # [doc = "     .event(expect::event())"] # [doc = "     .run_with_handle();"] # [doc = ""] # [doc = " let _subscriber =  tracing_subscriber::registry()"] # [doc = "     .with("] # [doc = "         layer_2.with_filter(tracing_subscriber::filter::filter_fn(move |_meta| true))"] # [doc = "     )"] # [doc = "     .set_default();"] # [doc = " {"] # [doc = "     let _subscriber =  tracing_subscriber::registry()"] # [doc = "         .with("] # [doc = "             layer_1"] # [doc = "                 .with_filter(tracing_subscriber::filter::filter_fn(move |_meta| true))"] # [doc = "         )"] # [doc = "         .set_default();"] # [doc = ""] # [doc = "     tracing::info!(\"a\");"] # [doc = " }"] # [doc = ""] # [doc = " handle_1.assert_finished();"] # [doc = " handle_2.assert_finished();"] # [doc = " ```"] # [doc = ""] # [doc = " [`MockLayerBuilder::named`]: fn@crate::layer::MockLayerBuilder::named"] # [doc = " [`layer`]: mod@crate::layer"] # [must_use] pub fn named (name : impl std :: fmt :: Display) -> MockLayerBuilder { mock () . named (name) }
    };
}

named!();