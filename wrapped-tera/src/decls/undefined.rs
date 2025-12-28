macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! undefined {
    () => {
        deps!();
        # [doc = " Returns true if `value` is undefined. Otherwise, returns false."] pub fn undefined (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("undefined" , 0 , params . len ()) ? ; Ok (value . is_none ()) }
    };
}

undefined!()