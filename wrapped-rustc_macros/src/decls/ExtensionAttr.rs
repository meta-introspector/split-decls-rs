macro_rules! ExtensionAttr {
    () => {
        struct ExtensionAttr { vis : Visibility , trait_ : Path , }
    };
}

ExtensionAttr!()