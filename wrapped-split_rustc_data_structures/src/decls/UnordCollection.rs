macro_rules! deps {
    () => {
        UnordItems!();
    };
}

macro_rules! UnordCollection {
    () => {
        deps!();
        # [doc = " A marker trait specifying that `Self` can consume `UnordItems<_>` without"] # [doc = " exposing any internal ordering."] # [doc = ""] # [doc = " Note: right now this is just a marker trait. It could be extended to contain"] # [doc = " some useful, common methods though, like `len`, `clear`, or the various"] # [doc = " kinds of `to_sorted`."] trait UnordCollection { }
    };
}

UnordCollection!()