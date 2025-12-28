macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ending_with {
    () => {
        deps!();
        # [doc = " Returns true if `value` ends with the given string. Otherwise, returns false."] pub fn ending_with (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("ending_with" , 1 , params . len ()) ? ; value_defined ("ending_with" , value) ? ; let value = extract_string ("ending_with" , "on a variable" , value) ? ; let needle = extract_string ("ending_with" , "with a parameter" , params . first ()) ? ; Ok (value . ends_with (needle)) }
    };
}

ending_with!()