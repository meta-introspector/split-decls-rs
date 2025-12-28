macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! odd {
    () => {
        deps!();
        # [doc = " Returns true if `value` is an odd number. Otherwise, returns false."] pub fn odd (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("odd" , 0 , params . len ()) ? ; value_defined ("odd" , value) ? ; match value . and_then (| v | v . to_number () . ok ()) { Some (f) => Ok (f % 2.0 != 0.0) , _ => Err (Error :: msg ("Tester `odd` was called on a variable that isn't a number")) , } }
    };
}

odd!()