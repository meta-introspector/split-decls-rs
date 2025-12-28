macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! MultiName {
    () => {
        deps!();
        pub struct MultiName { pub (crate) serialize : Name , pub (crate) serialize_renamed : bool , pub (crate) deserialize : Name , pub (crate) deserialize_renamed : bool , pub (crate) deserialize_aliases : BTreeSet < Name > , }
    };
}

MultiName!()