macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! has_flatten {
    () => {
        deps!();
        # [doc = " True if there is any field with a `#[serde(flatten)]` attribute, other than"] # [doc = " fields which are skipped."] fn has_flatten (fields : & [Field]) -> bool { fields . iter () . any (| field | field . attrs . flatten () && ! field . attrs . skip_deserializing ()) }
    };
}

has_flatten!();