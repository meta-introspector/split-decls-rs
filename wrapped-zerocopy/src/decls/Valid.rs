macro_rules! Valid {
    () => {
        # [doc = " The referent of a `Ptr<T>` is valid for `T`, upholding bit validity and any"] # [doc = " library safety invariants."] pub enum Valid { }
    };
}

Valid!()