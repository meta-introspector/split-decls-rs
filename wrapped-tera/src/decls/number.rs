macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! number {
    () => {
        deps!();
        # [doc = " Returns true if `value` is a number. Otherwise, returns false."] pub fn number (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("number" , 0 , params . len ()) ? ; value_defined ("number" , value) ? ; match value { Some (Value :: Number (_)) => Ok (true) , _ => Ok (false) , } }
    };
}

number!()