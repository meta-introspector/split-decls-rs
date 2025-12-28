macro_rules! deps {
    () => {
        Layer!();
        LookupSpan!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [doc = " Represents information about the current context provided to [`Layer`]s by the"] # [doc = " wrapped [`Subscriber`]."] # [doc = ""] # [doc = " To access [stored data] keyed by a span ID, implementors of the `Layer`"] # [doc = " trait should ensure that the `Subscriber` type parameter is *also* bound by the"] # [doc = " [`LookupSpan`]:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use tracing::Subscriber;"] # [doc = " use tracing_subscriber::{Layer, registry::LookupSpan};"] # [doc = ""] # [doc = " pub struct MyLayer;"] # [doc = ""] # [doc = " impl<S> Layer<S> for MyLayer"] # [doc = " where"] # [doc = "     S: Subscriber + for<'a> LookupSpan<'a>,"] # [doc = " {"] # [doc = "     // ..."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`Layer`]: super::Layer"] # [doc = " [`Subscriber`]: tracing_core::Subscriber"] # [doc = " [stored data]: crate::registry::SpanRef"] # [doc = " [`LookupSpan`]: crate::registry::LookupSpan"] # [derive (Debug)] pub struct Context < 'a , S > { subscriber : Option < & 'a S > , # [doc = " The bitmask of all [`Filtered`] layers that currently apply in this"] # [doc = " context. If there is only a single [`Filtered`] wrapping the layer that"] # [doc = " produced this context, then this is that filter's ID. Otherwise, if we"] # [doc = " are in a nested tree with multiple filters, this is produced by"] # [doc = " [`and`]-ing together the [`FilterId`]s of each of the filters that wrap"] # [doc = " the current layer."] # [doc = ""] # [doc = " [`Filtered`]: crate::filter::Filtered"] # [doc = " [`FilterId`]: crate::filter::FilterId"] # [doc = " [`and`]: crate::filter::FilterId::and"] # [cfg (all (feature = "registry" , feature = "std"))] filter : FilterId , }
    };
}

Context!();