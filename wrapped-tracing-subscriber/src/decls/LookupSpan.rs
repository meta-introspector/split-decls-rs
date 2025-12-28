macro_rules! deps {
    () => {
        SpanData!();
        Context!();
        Layer!();
        SpanRef!();
    };
}

macro_rules! LookupSpan {
    () => {
        deps!();
        # [doc = " Provides access to stored span data."] # [doc = ""] # [doc = " Subscribers which store span data and associate it with span IDs should"] # [doc = " implement this trait; if they do, any [`Layer`]s wrapping them can look up"] # [doc = " metadata via the [`Context`] type's [`span()`] method."] # [doc = ""] # [doc = " [`Layer`]: super::layer::Layer"] # [doc = " [`Context`]: super::layer::Context"] # [doc = " [`span()`]: super::layer::Context::span"] pub trait LookupSpan < 'a > { # [doc = " The type of span data stored in this registry."] type Data : SpanData < 'a > ; # [doc = " Returns the [`SpanData`] for a given `Id`, if it exists."] # [doc = ""] # [doc = " <pre class=\"ignore\" style=\"white-space:normal;font:inherit;\">"] # [doc = " <strong>Note</strong>: users of the <code>LookupSpan</code> trait should"] # [doc = " typically call the <a href=\"#method.span\"><code>span</code></a> method rather"] # [doc = " than this method. The <code>span</code> method is implemented by"] # [doc = " <em>calling</em> <code>span_data</code>, but returns a reference which is"] # [doc = " capable of performing more sophisiticated queries."] # [doc = " </pre>"] # [doc = ""] fn span_data (& 'a self , id : & Id) -> Option < Self :: Data > ; # [doc = " Returns a [`SpanRef`] for the span with the given `Id`, if it exists."] # [doc = ""] # [doc = " A `SpanRef` is similar to [`SpanData`], but it allows performing"] # [doc = " additional lookups against the registryr that stores the wrapped data."] # [doc = ""] # [doc = " In general, _users_ of the `LookupSpan` trait should use this method"] # [doc = " rather than the [`span_data`] method; while _implementors_ of this trait"] # [doc = " should only implement `span_data`."] # [doc = ""] # [doc = " [`span_data`]: LookupSpan::span_data()"] fn span (& 'a self , id : & Id) -> Option < SpanRef < 'a , Self > > where Self : Sized , { let data = self . span_data (id) ? ; Some (SpanRef { registry : self , data , # [cfg (feature = "registry")] filter : FilterId :: none () , }) } # [doc = " Registers a [`Filter`] for [per-layer filtering] with this"] # [doc = " [`Subscriber`]."] # [doc = ""] # [doc = " The [`Filter`] can then use the returned [`FilterId`] to"] # [doc = " [check if it previously enabled a span][check]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If this `Subscriber` does not support [per-layer filtering]."] # [doc = ""] # [doc = " [`Filter`]: crate::layer::Filter"] # [doc = " [per-layer filtering]: crate::layer::Layer#per-layer-filtering"] # [doc = " [`Subscriber`]: tracing_core::Subscriber"] # [doc = " [`FilterId`]: crate::filter::FilterId"] # [doc = " [check]: SpanData::is_enabled_for"] # [cfg (feature = "registry")] # [cfg_attr (docsrs , doc (cfg (feature = "registry")))] fn register_filter (& mut self) -> FilterId { panic ! ("{} does not currently support filters" , std :: any :: type_name ::< Self > ()) } }
    };
}

LookupSpan!();