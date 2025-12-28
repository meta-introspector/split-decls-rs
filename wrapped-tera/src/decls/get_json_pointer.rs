macro_rules! get_json_pointer {
    () => {
        # [doc = " Converts a dotted path to a json pointer one"] # [inline] # [deprecated (since = "1.8.0" , note = "`get_json_pointer` converted a dotted pointer to a json pointer, use dotted_pointer for direct lookups of values")] pub fn get_json_pointer (key : & str) -> String { lazy_static :: lazy_static ! { static ref JSON_POINTER_REGEX : regex :: Regex = regex :: Regex :: new (r#""[^"]*"|[^.]+"#) . unwrap () ; } let mut res = String :: with_capacity (key . len () + 1) ; if key . find ('"') . is_some () { for mat in JSON_POINTER_REGEX . find_iter (key) { res . push ('/') ; res . push_str (mat . as_str () . trim_matches ('"')) ; } } else { res . push ('/') ; res . push_str (& key . replace ('.' , "/")) ; } res }
    };
}

get_json_pointer!();