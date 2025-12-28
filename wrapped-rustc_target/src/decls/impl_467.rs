macro_rules! deps {
    () => {
        ToJson!();
        LinkSelfContainedDefault!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl ToJson for LinkSelfContainedDefault { fn to_json (& self) -> Json { match * self { LinkSelfContainedDefault :: WithComponents (components) => { let mut map = BTreeMap :: new () ; map . insert ("components" , components) ; map . to_json () } LinkSelfContainedDefault :: True => "true" . to_json () , LinkSelfContainedDefault :: False => "false" . to_json () , LinkSelfContainedDefault :: InferredForMusl => "musl" . to_json () , LinkSelfContainedDefault :: InferredForMingw => "mingw" . to_json () , } } }
    };
}

impl_467!()