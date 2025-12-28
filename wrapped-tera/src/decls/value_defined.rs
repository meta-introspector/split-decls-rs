macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! value_defined {
    () => {
        deps!();
        # [doc = " Called to check if the Value is defined and return an Err if not"] pub fn value_defined (tester_name : & str , value : Option < & Value >) -> Result < () > { if value . is_none () { return Err (Error :: msg (format ! ("Tester `{}` was called on an undefined variable" , tester_name))) ; } Ok (()) }
    };
}

value_defined!()