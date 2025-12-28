macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! object {
    () => {
        deps!();
        # [doc = " Returns true if the given variable is an object (ie can be iterated over key, value)."] # [doc = " Otherwise, returns false."] pub fn object (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("object" , 0 , params . len ()) ? ; value_defined ("object" , value) ? ; Ok (value . unwrap () . is_object ()) }
    };
}

object!();