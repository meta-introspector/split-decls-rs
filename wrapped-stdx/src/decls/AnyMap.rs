macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! AnyMap {
    () => {
        deps!();
        # [doc = " The most common type of `Map`: just using `Any`; `[Map]<dyn [Any]>`."] # [doc = ""] # [doc = " Why is this a separate type alias rather than a default value for `Map<A>`?"] # [doc = " `Map::default()` doesn't seem to be happy to infer that it should go with the default"] # [doc = " value. It's a bit sad, really. Ah well, I guess this approach will do."] pub type AnyMap = Map < dyn Any > ;
    };
}

AnyMap!();