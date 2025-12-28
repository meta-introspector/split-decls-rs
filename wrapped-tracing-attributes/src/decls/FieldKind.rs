macro_rules! FieldKind {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq)] pub (crate) enum FieldKind { Debug , Display , Value , }
    };
}

FieldKind!()