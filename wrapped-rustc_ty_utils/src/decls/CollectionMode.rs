macro_rules! CollectionMode {
    () => {
        enum CollectionMode { # [doc = " For impl trait in assoc types we only permit collecting them from"] # [doc = " associated types of the same impl block."] ImplTraitInAssocTypes , # [doc = " When collecting for an explicit `#[define_opaque]` attribute, find all TAITs"] Taits , # [doc = " The default case, only collect RPITs and AsyncFn return types, as these are"] # [doc = " always defined by the current item."] RpitAndAsyncFnOnly , }
    };
}

CollectionMode!()