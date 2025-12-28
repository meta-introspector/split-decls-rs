macro_rules! Aligned {
    () => {
        # [doc = " The referent is aligned: for `Ptr<T>`, the referent's address is a multiple"] # [doc = " of the `T`'s alignment."] pub enum Aligned { }
    };
}

Aligned!()