macro_rules! normalize_json_string {
    () => {
        # [cfg (feature = "structured-data")] fn normalize_json_string (value : & mut serde_json :: Value , op : & dyn Fn (& str) -> String) { match value { serde_json :: Value :: String (str) => { * str = op (str) ; } serde_json :: Value :: Array (arr) => { for value in arr . iter_mut () { normalize_json_string (value , op) ; } } serde_json :: Value :: Object (obj) => { for (key , mut value) in std :: mem :: replace (obj , serde_json :: Map :: new ()) { let key = op (& key) ; normalize_json_string (& mut value , op) ; obj . insert (key , value) ; } } _ => { } } }
    };
}

normalize_json_string!()