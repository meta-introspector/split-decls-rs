macro_rules! DocFakeItemKind {
    () => {
        # [derive (Clone , Copy)] enum DocFakeItemKind { Attribute , Keyword , }
    };
}

DocFakeItemKind!();