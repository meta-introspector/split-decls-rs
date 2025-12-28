macro_rules! Internal {
    () => {
        # [doc = " A marker for restricting a method on a public trait to internal use only."] pub (crate) enum Internal { }
    };
}

Internal!()