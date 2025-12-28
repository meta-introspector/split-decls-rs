macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! string {
    () => {
        deps!();
        # [doc = " Returns true if `value` is a string. Otherwise, returns false."] pub fn string (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("string" , 0 , params . len ()) ? ; value_defined ("string" , value) ? ; match value { Some (Value :: String (_)) => Ok (true) , _ => Ok (false) , } }
    };
}

string!();