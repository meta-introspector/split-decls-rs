macro_rules! Object {
    () => {
        type Object = serde_json :: Map < String , JsonValue > ;
    };
}

Object!()