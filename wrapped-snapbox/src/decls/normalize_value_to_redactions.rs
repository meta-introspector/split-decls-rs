macro_rules! deps {
    () => {
        Redactions!();
    };
}

macro_rules! normalize_value_to_redactions {
    () => {
        deps!();
        # [cfg (feature = "structured-data")] fn normalize_value_to_redactions (actual : & mut serde_json :: Value , expected : & serde_json :: Value , substitutions : & Redactions ,) { use serde_json :: Value :: { Array , Object , String } ; match (actual , expected) { (act , String (exp)) if exp == VALUE_WILDCARD => { * act = serde_json :: json ! (VALUE_WILDCARD) ; } (String (act) , String (exp)) => { * act = normalize_str_to_redactions (act , exp , substitutions) ; } (Array (act) , Array (exp)) => { * act = normalize_array_to_redactions (act , exp , substitutions) ; } (Object (act) , Object (exp)) => { let has_key_wildcard = exp . get (KEY_WILDCARD) . and_then (| v | v . as_str ()) == Some (VALUE_WILDCARD) ; for (actual_key , mut actual_value) in std :: mem :: replace (act , serde_json :: Map :: new ()) { if let Some (expected_value) = exp . get (& actual_key) { normalize_value_to_redactions (& mut actual_value , expected_value , substitutions) ; } else if has_key_wildcard { continue ; } act . insert (actual_key , actual_value) ; } if has_key_wildcard { act . insert (KEY_WILDCARD . to_owned () , String (VALUE_WILDCARD . to_owned ())) ; } } (_ , _) => { } } }
    };
}

normalize_value_to_redactions!()