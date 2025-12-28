macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! defined {
    () => {
        deps!();
        # [doc = " Returns true if `value` is defined. Otherwise, returns false."] pub fn defined (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("defined" , 0 , params . len ()) ? ; Ok (value . is_some ()) }
    };
}

defined!();