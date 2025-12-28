macro_rules! deps {
    () => {
        LookupSpan!();
    };
}

macro_rules! SpanData {
    () => {
        deps!();
        # [doc = " A stored representation of data associated with a span."] pub trait SpanData < 'a > { # [doc = " Returns this span's ID."] fn id (& self) -> Id ; # [doc = " Returns a reference to the span's `Metadata`."] fn metadata (& self) -> & 'static Metadata < 'static > ; # [doc = " Returns a reference to the ID"] fn parent (& self) -> Option < & Id > ; # [doc = " Returns a reference to this span's `Extensions`."] # [doc = ""] # [doc = " The extensions may be used by `Layer`s to store additional data"] # [doc = " describing the span."] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] fn extensions (& self) -> Extensions < '_ > ; # [doc = " Returns a mutable reference to this span's `Extensions`."] # [doc = ""] # [doc = " The extensions may be used by `Layer`s to store additional data"] # [doc = " describing the span."] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] fn extensions_mut (& self) -> ExtensionsMut < '_ > ; # [doc = " Returns `true` if this span is enabled for the [per-layer filter][plf]"] # [doc = " corresponding to the provided [`FilterId`]."] # [doc = ""] # [doc = " ## Default Implementation"] # [doc = ""] # [doc = " By default, this method assumes that the [`LookupSpan`] implementation"] # [doc = " does not support [per-layer filtering][plf], and always returns `true`."] # [doc = ""] # [doc = " [plf]: crate::layer::Layer#per-layer-filtering"] # [doc = " [`FilterId`]: crate::filter::FilterId"] # [cfg (feature = "registry")] # [cfg_attr (docsrs , doc (cfg (feature = "registry")))] fn is_enabled_for (& self , filter : FilterId) -> bool { let _ = filter ; true } }
    };
}

SpanData!();