macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! matching {
    () => {
        deps!();
        # [doc = " Returns true if `value` is a string and matches the regex in the argument. Otherwise, returns false."] pub fn matching (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("matching" , 1 , params . len ()) ? ; value_defined ("matching" , value) ? ; let value = extract_string ("matching" , "on a variable" , value) ? ; let regex = extract_string ("matching" , "with a parameter" , params . first ()) ? ; let regex = match Regex :: new (regex) { Ok (regex) => regex , Err (err) => { return Err (Error :: msg (format ! ("Tester `matching`: Invalid regular expression: {}" , err))) ; } } ; Ok (regex . is_match (value)) }
    };
}

matching!();