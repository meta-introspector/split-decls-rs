macro_rules! deps {
    () => {
        RawMap!();
        AnyMap!();
        Downcast!();
    };
}

macro_rules! Map {
    () => {
        deps!();
        # [doc = " A collection containing zero or one values for any given type and allowing convenient,"] # [doc = " type-safe access to those values."] # [doc = ""] # [doc = " The type parameter `A` allows you to use a different value type; normally you will want"] # [doc = " it to be `core::any::Any` (also known as `std::any::Any`), but there are other choices:"] # [doc = ""] # [doc = " - You can add on `+ Send` or `+ Send + Sync` (e.g. `Map<dyn Any + Send>`) to add those"] # [doc = "   auto traits."] # [doc = ""] # [doc = " Cumulatively, there are thus six forms of map:"] # [doc = ""] # [doc = " - `[Map]<dyn [core::any::Any]>`,"] # [doc = "   also spelled [`AnyMap`] for convenience."] # [doc = " - `[Map]<dyn [core::any::Any] + Send>`"] # [doc = " - `[Map]<dyn [core::any::Any] + Send + Sync>`"] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " (Here, the [`AnyMap`] convenience alias is used;"] # [doc = " the first line could use `[anymap::Map][Map]::<[core::any::Any]>::default()`"] # [doc = " instead if desired.)"] # [doc = ""] # [doc = " ```"] # [doc = " # use stdx::anymap;"] # [doc = " let mut data = anymap::AnyMap::default();"] # [doc = " assert_eq!(data.get(), None::<&i32>);"] # [doc = " ```"] # [doc = ""] # [doc = " Values containing non-static references are not permitted."] # [derive (Debug)] pub struct Map < A : ? Sized + Downcast = dyn Any > { raw : RawMap < A > , }
    };
}

Map!()