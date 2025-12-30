// Generated macro for ChromeLayerBuilder (struct)
macro_rules! DepcrateChromeLayerBuilder {
() => {
// Module: crate
// Provides: {"ChromeLayerBuilder"}
// Dependencies: {}
# [doc = " A builder for [`ChromeLayer`](crate::ChromeLayer)."] # [derive (Default)] pub struct ChromeLayerBuilder < S > where S : Subscriber + for < 'span > LookupSpan < 'span > + Send + Sync , { out_writer : Option < Box < dyn Write + Send > > , name_fn : Option < NameFn < S > > , cat_fn : Option < NameFn < S > > , include_args : bool , include_locations : bool , trace_style : TraceStyle , _inner : PhantomData < S > , }
};
}
