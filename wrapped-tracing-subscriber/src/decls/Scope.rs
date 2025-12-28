macro_rules! Scope {
    () => {
        # [doc = " An iterator over the parents of a span, ordered from leaf to root."] # [doc = ""] # [doc = " This is returned by the [`SpanRef::scope`] method."] # [derive (Debug)] pub struct Scope < 'a , R > { registry : & 'a R , next : Option < Id > , # [cfg (all (feature = "registry" , feature = "std"))] filter : FilterId , }
    };
}

Scope!()