macro_rules! deps {
    () => {
        DocFakeItemKind!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl DocFakeItemKind { fn name (self) -> & 'static str { match self { Self :: Attribute => "attribute" , Self :: Keyword => "keyword" , } } }
    };
}

impl_12!();